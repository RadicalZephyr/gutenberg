use ast::{Content, Node};
use pulldown_cmark::{Event, Tag};

pub trait IntoHtml<C> {
    fn render(&mut self, ctx: &mut C, buf: &mut String);
}

enum TagType {
    Opening,
    Closing,
    None,
}

struct Context {
    tag_type: TagType,
}

impl Context {
    fn new() -> Context {
        Context {
            tag_type: TagType::None,
        }
    }

    fn render_tag(&self, tag: &str, buf: &mut String) {
        let tag_closer = match self.tag_type {
            TagType::Closing => "/",
            _ => "",
        };
        buf.push_str(&format!("<{}{}>", tag_closer, tag));
    }
}

impl<'a> IntoHtml<Context> for Tag<'a> {
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        match *self {
            Tag::Paragraph => context.render_tag("p", buf),
            Tag::Header(n) => context.render_tag(&format!("h{}", n), buf),
            _ => (),
        }
    }
}

impl<'a> IntoHtml<Context> for Event<'a> {
    fn render(&mut self, _context: &mut Context, buf: &mut String) {
        match *self {
            Event::Start(_) | Event::End(_) => unreachable!(),
            Event::Text(ref text) => buf.push_str(text),
            _ => panic!("AHHHHHHH!!!!!!!!!!"),
        }
    }
}

impl<'a> IntoHtml<Context> for Node<'a> {
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        match *self {
            Node::Block(ref mut tag, ref mut content) => {
                context.tag_type = TagType::Opening;
                tag.render(context, buf);

                context.tag_type = TagType::None;
                content.render(context, buf);

                context.tag_type = TagType::Closing;
                tag.render(context, buf);
                buf.push('\n');
                context.tag_type = TagType::None;
            },
            Node::Item(ref mut event) => event.render(context, buf),
        }

    }
}

impl<'a, I> IntoHtml<Context> for Content<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        for mut node in self {
            node.render(context, buf);
        }
    }
}

pub fn into_html<'a, I>(content: &mut Content<'a, I>, buf: &mut String)
where
    I: Iterator<Item = Event<'a>>
{
    let mut context = Context::new();
    content.render(&mut context, buf);
}
