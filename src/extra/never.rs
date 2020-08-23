// Poor mans never.

#[derive(Debug)]
pub struct Never<'a> {
    never: &'a Never<'a>,
}
