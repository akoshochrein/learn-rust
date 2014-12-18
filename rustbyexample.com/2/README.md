
Not surprisingly, the print macro is there and alive.

```Rust
print!("this will appear in the same line ");
println!("as this");
```

When you add a constant into this formatter, you need to specify the type.
Examples for this are quite rad. For example you need to specift how big of a float it is when you try to do a float value.

```Rust
println!("{}", 67.2f32);
println!("{}", 67.2f64);
```

These brass moles yield different values:

67.199997
67.2

What the f. I only added 1 number after the decimal and it broke? I love this language! This proves that 32 bit floats are crap.

[Holy shit how did I end here](http://bluishcoder.co.nz/2013/08/15/phantom_types_in_rust.html)

Back to formatting... Actually this works a bit like in Python. If I declare the formatting elements as variables, I can assign them as kwargs. Pretty darn cool.

```Rust
println!("{first_name} {last_name} is {attribute}", first_name="Liana", last_name="Lo", attriute="cool");
```

This will make logging pretty easy to get used to... I mean if there's IO in Rust.

Mixing things up is good for us as well!

```Rust
println!("{stuff}, {}, {1}", 1i, 2i, stuff=3i);
```

This is enough for formatting, let's see what happens in the next chapter.
