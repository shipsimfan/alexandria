use crate::{
    compile::{
        ast::items::Struct as AstStruct, semantic_analysis::type_solving::solve_type_by_name, Ast,
    },
    program::{
        types::{Struct, StructField, StructFieldMeta, TypeManager},
        Type,
    },
};
use lct_diagnostics::{Diag, DiagCtxt};
use std::rc::Rc;

/// Solve a struct and any types the struct relies on
pub(super) fn solve<'a, 'b, 'c>(
    ast: &Ast<'c>,
    r#struct: &AstStruct<'c>,
    in_progress_types: &mut Vec<&'c str>,
    types: &mut TypeManager,
    diag: &'b DiagCtxt<'a>,
) -> Result<Rc<Type>, Diag<'a, 'b>> {
    let mut fields = Vec::new();
    let mut used_field_meta = Vec::new();
    for (i, field) in r#struct.fields().iter().enumerate() {
        // Validate the field name is unique
        for other_field in &r#struct.fields()[..i] {
            if field.name() == other_field.name() {
                return Err(diag.err_span(
                    format!("field `{}` is already defined", field.name()),
                    field.span(),
                ));
            }
        }

        // Validate the field attributes are unique and valid
        if field.attributes().len() > 1 {
            return Err(diag.err_span(
                format!("cannot have more than attribute on a struct field"),
                field.attributes()[1].span(),
            ));
        }

        let meta = if let Some(attribute) = field.attributes().first() {
            let new_meta = match attribute.name() {
                "position" => StructFieldMeta::Position,
                _ => {
                    return Err(diag.err_span(
                        format!("unknown struct field attribute `{}`", attribute.name()),
                        attribute.span(),
                    ))
                }
            };

            for meta in &used_field_meta {
                if *meta == new_meta {
                    return Err(diag.err_span(
                        format!("attribute `{}` is already used in this struct", meta),
                        attribute.span(),
                    ));
                }
            }

            used_field_meta.push(new_meta);
            Some(new_meta)
        } else {
            None
        };

        // Solve the type
        let r#type = solve_type_by_name(
            field.span(),
            field.r#type(),
            in_progress_types,
            types,
            ast,
            diag,
        )?;

        // Add the field
        fields.push(StructField::new(meta, field.name().to_string(), r#type));
    }

    Ok(types.add(Struct::new(
        r#struct.name().content().to_string(),
        r#struct.id(),
        fields,
    )))
}
