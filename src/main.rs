mod mathlan;

use mathlan::execution::{execute, parse};

fn modify_text(text: &mut str) {
    text.make_ascii_lowercase();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let program = parse(r#"
        35 34 + print
        72 2 / 2 * print
        2 2 exp print
        1 2 < print
        1 2 = print
        1 2 > print
        1 1 = if 100 print else 200 print end
        1 2 = if 101 print else 202 print end
        1 dup print 2 + print
        1 2 - print 1 2 swap - print
    "#)?;*/

    /*
        let a = 10
        if a > 0
            print a
            a--
            jump to if
        else
        // print "Done"
        putc 68 
        putc 111 
        putc 110 
        putc 101 
        end
    */
    let program = parse(r#"
        10 dup 0 > if 
        dup print 1 - -9 
        jump 
        else 
        68 putc 111 putc 110 putc 101 putc 10 putc
        end
    "#)?;
    
    for (index, operation) in program.iter().enumerate() {
        println!("{:02}: {:?}", index, operation);
    }
    execute(&program)?;

    Ok(())
}
