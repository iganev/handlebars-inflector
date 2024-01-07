use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
    RenderErrorReason,
};
use inflector::Inflector;

#[derive(Clone, Copy)]
/// Inflector helper for handlebars-rust
///
/// # Registration
///
/// ```
/// use handlebars::Handlebars;
/// use handlebars_inflector::HandlebarsInflector;
///
/// let mut h = Handlebars::new();
/// h.register_helper("inflect", Box::new(HandlebarsInflector));
///
/// assert_eq!(h.render_template(r#"{{inflect this to_singular=true}}"#, &String::from("tests")).expect("Render error"), "test");
/// ```
///
/// # Arguments
///
/// * `param` - A string value to be used for inflection
///
/// # Example usage:
///
/// `camelCase`:
///
/// `
/// {{inflect param to_camel_case=true}}
/// `
///
/// `Product`:
///
/// `
/// {{inflect param to_class_case=true}}
/// `
///
/// `PascalCase`:
///
/// `
/// {{inflect param to_pascal_case=true}}
/// `
///
/// `product description`:
///
/// `
/// {{inflect param to_singular=true to_sentence_case=true to_lower_case=true }}
/// `
///
/// # Operations
///
/// List of possible operations in the order of execution:
///
/// `to_camel_case`: `product_images` to `productImages`
///
/// `to_pascal_case`: `product_images` to `ProductImages`
///
/// `to_snake_case`: `ProductImages` to `product_images`
///
/// `to_screaming_snake_case`: `ProductImages` to `PRODUCT_IMAGES`
///
/// `to_kebab_case`: `product_images` to `product-images`
///
/// `to_train_case`: `product_images` to `Product-Images`
///
/// `to_sentence_case`: `product_images` to `Product images`
///
/// `to_title_case`: `product_images` to `Product Images`
///
/// `ordinalize`: `July 1` to `July 1st`
///
/// `deordinalize`: `July 1st` to `July 1`
///
/// `to_foreign_key`: `Product image` to `product_image_id`
///
/// `demodulize`: `std::io` to `Io`
///
/// `deconstantize`: `std::io` to `Std`
///
/// `to_class_case`: `product_images` to `ProductImage`
///
/// `to_table_case`: `ProductImage` to `product_images`
///
/// `to_plural`: `ProductImage` to `ProductImages`
///
/// `to_singular`: `product_images` to `product_image`
///
/// `to_upper_case`: `product_images` to `PRODUCT_IMAGES`
///
/// `to_lower_case`: `ProductImages` to `productimages`
///
/// Note that some combinations might not be compatible with one another.
/// If you still absolutely need to do such type of combination you could nest the operations like:
///
/// `Bars::Foos` to `Bar`
///
/// `
/// {{inflect (inflect param deconstantize=true) to_singular=true}}
/// `
///
pub struct HandlebarsInflector;

impl HelperDef for HandlebarsInflector {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        r: &'reg Handlebars,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let input = if let Some(input) = h.param(0) {
            input
        } else {
            if r.strict_mode() {
                return Err(RenderErrorReason::ParamNotFoundForIndex("inflect", 0).into());
            }

            return Ok(());
        };

        if !input.value().is_string() {
            if r.strict_mode() {
                return Err(RenderErrorReason::ParamTypeMismatchForName(
                    "inflect",
                    "0".to_string(),
                    "string".to_string(),
                )
                .into());
            }

            return Ok(());
        }

        let mut output = input.value().render();

        if h.hash_get("to_camel_case").is_some() {
            output = output.to_camel_case();
        }

        if h.hash_get("to_pascal_case").is_some() {
            output = output.to_pascal_case();
        }

        if h.hash_get("to_snake_case").is_some() {
            output = output.to_snake_case();
        }

        if h.hash_get("to_screaming_snake_case").is_some() {
            output = output.to_screaming_snake_case();
        }

        if h.hash_get("to_kebab_case").is_some() {
            output = output.to_kebab_case();
        }

        if h.hash_get("to_train_case").is_some() {
            output = output.to_train_case();
        }

        if h.hash_get("to_sentence_case").is_some() {
            output = output.to_sentence_case();
        }

        if h.hash_get("to_title_case").is_some() {
            output = output.to_title_case();
        }

        if h.hash_get("ordinalize").is_some() {
            output = output.ordinalize();
        }

        if h.hash_get("deordinalize").is_some() {
            output = output.deordinalize();
        }

        if h.hash_get("to_foreign_key").is_some() {
            output = output.to_foreign_key();
        }

        if h.hash_get("demodulize").is_some() {
            output = output.demodulize();
        }

        if h.hash_get("deconstantize").is_some() {
            output = output.deconstantize();
        }

        if h.hash_get("to_class_case").is_some() {
            output = output.to_class_case();
        }

        if h.hash_get("to_table_case").is_some() {
            output = output.to_table_case();
        }

        if h.hash_get("to_plural").is_some() {
            output = output.to_plural();
        }

        if h.hash_get("to_singular").is_some() {
            output = output.to_singular();
        }

        if h.hash_get("to_upper_case").is_some() {
            output = output.to_uppercase();
        }

        if h.hash_get("to_lower_case").is_some() {
            output = output.to_lowercase();
        }

        out.write(&output)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;

    #[test]
    fn it_works() {
        let mut h = Handlebars::new();
        h.register_helper("inflect", Box::new(HandlebarsInflector));

        assert_eq!(
            h.render_template(
                r#"{{inflect this to_camel_case=true}}"#,
                &String::from("this is a test")
            )
            .expect("Render error"),
            "thisIsATest",
            "Failed to test to_camel_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_pascal_case=true}}"#,
                &String::from("product_images")
            )
            .expect("Render error"),
            "ProductImages",
            "Failed to test to_pascal_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_snake_case=true}}"#,
                &String::from("ProductImages")
            )
            .expect("Render error"),
            "product_images",
            "Failed to test to_snake_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_screaming_snake_case=true}}"#,
                &String::from("ProductImages")
            )
            .expect("Render error"),
            "PRODUCT_IMAGES",
            "Failed to test to_screaming_snake_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_kebab_case=true}}"#,
                &String::from("product_images")
            )
            .expect("Render error"),
            "product-images",
            "Failed to test to_kebab_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_sentence_case=true}}"#,
                &String::from("product_images")
            )
            .expect("Render error"),
            "Product images",
            "Failed to test to_sentence_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_title_case=true}}"#,
                &String::from("product_images")
            )
            .expect("Render error"),
            "Product Images",
            "Failed to test to_title_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this ordinalize=true}}"#,
                &String::from("July 1")
            )
            .expect("Render error"),
            "July 1st",
            "Failed to test ordinalize"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this deordinalize=true}}"#,
                &String::from("July 1st")
            )
            .expect("Render error"),
            "July 1",
            "Failed to test deordinalize"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_foreign_key=true}}"#,
                &String::from("ProductImage")
            )
            .expect("Render error"),
            "product_image_id",
            "Failed to test to_foreign_key"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this demodulize=true}}"#,
                &String::from("Foo::Bar")
            )
            .expect("Render error"),
            "Bar",
            "Failed to test demodulize"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this deconstantize=true}}"#,
                &String::from("Foo::Bar")
            )
            .expect("Render error"),
            "Foo",
            "Failed to test deconstantize"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_class_case=true}}"#,
                &String::from("product_images")
            )
            .expect("Render error"),
            "ProductImage",
            "Failed to test to_class_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_table_case=true}}"#,
                &String::from("ProductImage")
            )
            .expect("Render error"),
            "product_images",
            "Failed to test to_table_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_plural=true}}"#,
                &String::from("product image")
            )
            .expect("Render error"),
            "product images",
            "Failed to test to_plural"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_singular=true}}"#,
                &String::from("product_images")
            )
            .expect("Render error"),
            "product_image",
            "Failed to test to_singular"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_upper_case=true}}"#,
                &String::from("product image")
            )
            .expect("Render error"),
            "PRODUCT IMAGE",
            "Failed to test to_upper_case"
        );
        assert_eq!(
            h.render_template(
                r#"{{inflect this to_lower_case=true}}"#,
                &String::from("PRODUCT IMAGE")
            )
            .expect("Render error"),
            "product image",
            "Failed to test to_lower_case"
        );
    }
}
