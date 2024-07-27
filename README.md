[![Crates.io](https://img.shields.io/crates/v/handlebars-inflector?color=4d76ae)](https://crates.io/crates/handlebars-inflector)
[![API](https://docs.rs/handlebars-inflector/badge.svg)](https://docs.rs/handlebars-inflector)
[![build and test](https://github.com/iganev/handlebars-inflector/actions/workflows/rust.yml/badge.svg)](https://github.com/iganev/handlebars-inflector/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/iganev/handlebars-inflector/status.svg)](https://deps.rs/repo/github/iganev/handlebars-inflector)

# handlebars-inflector
String inflector helper for [handlebars-rust](https://github.com/sunng87/handlebars-rust)

## Quick Start

Developed and tested with handlebars-rust v4.4.0 and Inflector v0.11.4.  
Versions `0.1.*` are compatible with handlebars `4`.  
Versions `0.2.*` are compatible with handlebars `5`. (Thanks to [campeis](https://github.com/campeis))

### Registration

```rust
    use handlebars::Handlebars;
    use handlebars_inflector::HandlebarsInflector;
    
    let mut h = Handlebars::new();
    h.register_helper("inflect", Box::new(HandlebarsInflector));
```

### Usage

The helper is looking for exactly one parameter of type string. Operations can be stacked but are then executed  
in a specific predefined order. The operation parameter value doesn't matter as long as the parameter is  
present.

```handlebars
{{inflect param to_singular=true to_sentence_case=true to_lower_case=true }}
```

### Operations

List of possible operations in the order of execution:  
`to_camel_case`: `product_images` to `productImages`  
`to_pascal_case`: `product_images` to `ProductImages`  
`to_snake_case`: `ProductImages` to `product_images`  
`to_screaming_snake_case`: `ProductImages` to `PRODUCT_IMAGES`  
`to_kebab_case`: `product_images` to `product-images`  
`to_train_case`: `product_images` to `Product-Images`  
`to_sentence_case`: `product_images` to `Product images`  
`to_title_case`: `product_images` to `Product Images`  
`ordinalize`: `July 1` to `July 1st`  
`deordinalize`: `July 1st` to `July 1`  
`to_foreign_key`: `Product image` to `product_image_id`  
`demodulize`: `std::io` to `Io`  
`deconstantize`: `std::io` to `Std`  
`to_class_case`: `product_images` to `ProductImage`  
`to_table_case`: `ProductImage` to `product_images`  
`to_plural`: `ProductImage` to `ProductImages`  
`to_singular`: `product_images` to `product_image`  
`to_upper_case`: `product_images` to `PRODUCT_IMAGES`  
`to_lower_case`: `ProductImages` to `productimages`  
  
Note that some combinations might not be compatible with one another.  
If you still absolutely need to do such type of combination you could nest the operations like:  
`Bars::Foos` to `Bar`
  
```handlebars
{{inflect (inflect param deconstantize=true) to_singular=true}}
```

## Acknowledgements

This is basically a thin wrapper around the [Inflector](https://github.com/whatisinternet/inflector) crate. Kudos to it's developer.

## License

This library (handlebars-inflector) is open sourced under the BSD 2 License.  
P.S.: I don't really care about the license so I picked one of my dependencies' license. :)
