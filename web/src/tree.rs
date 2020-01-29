//! Parser for web html tokenstream
use crate::Error;
use elvis_core::Tree;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Extra html stream
#[derive(Debug)]
pub struct Extra {
    pub end: bool,
    pub pos: usize,
    pub tag: &'static str,
}

/// process of parsing children
#[derive(Eq, PartialEq)]
enum ChildrenProcess {
    BeginTag,
    CloseTag,
    None,
    Plain,
}

/// process of parsing tag
enum TagProcess {
    Attrs,
    Quote,
    None,
    Tag,
}

/// Deserialize Tree from html string
///
/// `attrs` field follows MDN doc [HTML attribute refference][1],
/// all values are `String` in "".
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#Boolean_Attributes
pub fn rde(
    h: &'static str,
    pre: Option<Weak<RefCell<Tree>>>,
) -> Result<(Rc<RefCell<Tree>>, Option<self::Extra>), Error> {
    let mut pos = 0_usize;
    if h.is_empty() {
        return Ok((Rc::new(RefCell::new(Tree::default())), None));
    } else if h.find("</").is_none() {
        return Ok((Rc::new(RefCell::new(self::plain(h, pre.clone()))), None));
    }

    // the return-will tree
    let tree = Rc::new(RefCell::new(Tree::default()));
    let tw = Rc::downgrade(&tree);
    let (tag, attrs) = self::tag(&h[pos..], &mut pos)?;

    // parse f*cking children
    let mut children: Vec<Rc<RefCell<Tree>>> = vec![];
    let mut cext = self::ch(&h[pos..], Some(tw.clone()), tag, &mut children)?;

    // parse parallel children
    pos += cext.pos;
    while !cext.end {
        cext = self::ch(&h[pos..], Some(tw.clone()), tag, &mut children)?;
        pos += cext.pos;
    }

    // communite with child parser
    let mut ext = None;
    if (pos + 1) != h.len() {
        ext = Some(self::Extra {
            end: false,
            pos: pos,
            tag: cext.tag,
        });
    }

    let mut bt = tree.borrow_mut();
    bt.pre = pre;
    bt.tag = tag;
    bt.attrs = attrs;
    bt.children = children;
    drop(bt);

    Ok((tree, ext))
}

/// push child from html stream
pub fn ch(
    cht: &'static str,
    pre: Option<Weak<RefCell<Tree>>>,
    tag: &'static str,
    children: &mut Vec<Rc<RefCell<Tree>>>,
) -> Result<Extra, Error> {
    let mut itag = tag;
    let mut process = ChildrenProcess::None;
    let (mut t, mut c) = ((0, 0), (0, 0));
    for (p, q) in cht.chars().enumerate() {
        match q {
            '<' => {
                if process == ChildrenProcess::Plain {
                    c.1 = p;
                }

                process = ChildrenProcess::BeginTag;
                t.0 = p;
                t.1 = p;
            }
            '/' => {
                if &cht[t.0..(t.0 + 1)] == "<" {
                    process = ChildrenProcess::CloseTag;
                } else if process != ChildrenProcess::Plain {
                    return Err(Error::DeserializeHtmlError(format!(
                        "children parse failed {}, cht: {}, process: {}",
                        &tag,
                        &cht,
                        &cht[t.0..(t.0 + 1)],
                    )));
                }
            }
            '>' => {
                t.1 = p;
                match process {
                    ChildrenProcess::BeginTag => {
                        let (tree, ext) = self::rde(&cht[t.0..], pre.clone())?;
                        children.push(tree);

                        // communite with father node
                        if let Some(cext) = ext {
                            return Ok(Extra {
                                end: false,
                                tag: cext.tag,
                                pos: cext.pos + t.0,
                            });
                        }
                    }
                    ChildrenProcess::CloseTag => {
                        // verify tag, trim:  "/ tag"
                        itag = &cht[(t.0 + 1)..t.1].trim()[1..].trim()[..];
                        if itag != tag {
                            return Err(Error::DeserializeHtmlError(format!(
                                "children parse failed {}, cht: {}, close_tag: {}",
                                &tag, &cht, &itag
                            )));
                        } else if !cht[c.0..c.1].is_empty() {
                            children
                                .push(Rc::new(RefCell::new(plain(&cht[c.0..c.1], pre.clone()))));
                        }

                        return Ok(Extra {
                            end: true,
                            pos: p,
                            tag: itag,
                        });
                    }
                    _ => {
                        // None and Plain
                    }
                }
            }
            x if !x.is_whitespace() => {
                match process {
                    ChildrenProcess::None => {
                        process = ChildrenProcess::Plain;
                        c.0 = p;
                        c.1 = p;
                    }
                    ChildrenProcess::Plain => {
                        c.1 = p;
                    }
                    _ => {
                        // tag conditions
                    }
                }
            }
            _ => {
                // invalid chars
            }
        }
    }
    Ok(Extra {
        end: true,
        pos: cht.len(),
        tag: itag,
    })
}

/// generate palin text
pub fn plain(h: &'static str, pre: Option<Weak<RefCell<Tree>>>) -> Tree {
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    attrs.insert("text", h);

    Tree {
        pre: pre.clone(),
        tag: "plain",
        attrs: attrs,
        children: vec![],
    }
}

/// parse html tag
pub fn tag(
    h: &'static str,
    pos: &mut usize,
) -> Result<(&'static str, HashMap<&'static str, &'static str>), Error> {
    let (mut t, mut k, mut v) = ((0, 0), (0, 0), (0, 0));
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    let mut process = TagProcess::None;
    for (p, q) in h.chars().enumerate() {
        match q {
            '<' => {
                process = TagProcess::Tag;
                t.0 = p + 1;
                t.1 = p + 1;
            }
            '>' => {
                match process {
                    TagProcess::Tag => t.1 = p,
                    TagProcess::Attrs => {
                        if !&h[k.0..k.1].trim().is_empty() {
                            attrs.insert(&h[k.0..k.1].trim(), &h[v.0..v.1].trim());
                        }
                    }
                    _ => {}
                }

                *pos = *pos + p + 1;
                return Ok((&h[t.0..t.1].trim(), attrs));
            }
            '"' => match process {
                TagProcess::Quote => {
                    process = TagProcess::Attrs;
                    v.1 = p;
                }
                _ => {
                    v.0 = p + 1;
                    v.1 = p + 1;
                    process = TagProcess::Quote;
                }
            },
            '=' => match process {
                TagProcess::Attrs => k.1 = p,
                _ => {
                    return Err(Error::DeserializeHtmlError(format!(
                        "html tag parse failed: {}, html: {}",
                        &h[t.0..t.1],
                        &h
                    )))
                }
            },
            x if x.is_whitespace() => match process {
                TagProcess::Tag => {
                    if h[t.0..t.1].trim().is_empty() {
                        t.1 = p;
                    } else {
                        process = TagProcess::Attrs;
                        k.0 = p + 1;
                        k.1 = p + 1;
                    }
                }
                TagProcess::Quote => {
                    v.1 = p;
                }
                TagProcess::Attrs => {
                    if (k.1 - k.0 != 0) && (v.1 - v.0 != 0) {
                        attrs.insert(&h[k.0..k.1].trim(), &h[v.0..v.1].trim());
                        k.0 = p;
                        k.1 = p;
                    }
                }
                _ => {}
            },
            x if !x.is_whitespace() => match process {
                TagProcess::Tag => {
                    t.1 = p + 1;
                }
                TagProcess::Quote => {
                    v.1 = p;
                }
                TagProcess::Attrs => {
                    if v.0 == 0 {
                        k.1 = p;
                    } else {
                        v.1 = p;
                    }
                }
                _ => {}
            },
            _ => {
                return Err(Error::DeserializeHtmlError(format!(
                    "html tag parse failed: {}, html: {}, char: {}",
                    &h[t.0..t.1],
                    &h,
                    &q
                )))
            }
        }
    }

    Err(Error::DeserializeHtmlError(format!(
        "html tag parse failed: {}, html: {}",
        &h[t.0..t.1],
        &h
    )))
}