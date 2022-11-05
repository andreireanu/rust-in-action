use std::cmp::{min, max};


fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    for (i, line) in haystack.lines().enumerate(){
        if line.contains(needle) {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }
    println!("{:?}", tags);
    if tags.len() == 0 {
        return
    }
    println!("Not returned");

    for (tag_idx, i) in tags.iter().enumerate() {
        let min_val = i.saturating_sub(ctx_lines); 
        let max_val = min(haystack.len(), *i  + ctx_lines ); 
        println!("{} - {}", min_val, max_val);
        for (idx, line) in haystack.lines().enumerate() {
            if min_val <= idx && idx <= max_val  {
                ctx[tag_idx].push((idx, String::from(line)));
            };
        }
    }
    println!("{:?}", ctx);

}
