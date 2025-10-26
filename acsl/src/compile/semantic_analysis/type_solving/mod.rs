use crate::{
    compile::{ast::Item, tokens::Identifier, Ast},
    program::{types::TypeManager, Type},
};
use lct_diagnostics::{Diag, DiagCtxt, Span};
use std::rc::Rc;

mod r#struct;

/// Solve the connections between types and fill the type manager
pub(super) fn solve_types<'a, 'b>(
    types: &mut TypeManager,
    ast: &Ast,
    diag: &'b DiagCtxt<'a>,
) -> Result<(), Diag<'a, 'b>> {
    solve_ast_item(None, &mut Vec::new(), types, ast, diag).map(|_| ())
}

/// Solves a type by the given name
fn solve_type_by_name<'a, 'b, 'c>(
    span: Span,
    name: &Identifier<'c>,
    in_progress_types: &mut Vec<&'c str>,
    types: &mut TypeManager,
    ast: &Ast<'c>,
    diag: &'b DiagCtxt<'a>,
) -> Result<Rc<Type>, Diag<'a, 'b>> {
    // Check for type in type manager
    if let Some(r#type) = types.get(name.content()) {
        return Ok(r#type.clone());
    }

    // Check for type in progress
    for r#type in in_progress_types.iter() {
        if name.content() == *r#type {
            return Err(diag.err_span(
                format!("recursive type `{}` has infinite size", name.content()),
                span,
            ));
        }
    }

    // Check for type in ast
    solve_ast_item(Some(name.content()), in_progress_types, types, ast, diag)?.ok_or_else(|| {
        diag.err_span(
            format!("cannot find type `{}`", name.content()),
            name.span(),
        )
    })
}

/// Solves a type by the given name or all uncompleted types
fn solve_ast_item<'a, 'b, 'c>(
    name: Option<&'c str>,
    in_progress_types: &mut Vec<&'c str>,
    types: &mut TypeManager,
    ast: &Ast<'c>,
    diag: &'b DiagCtxt<'a>,
) -> Result<Option<Rc<Type>>, Diag<'a, 'b>> {
    for item in ast.items() {
        // Check for a type definition
        let (type_name, ast_id) = match is_type(item) {
            Some(info) => info,
            None => continue,
        };

        if let Some(name) = name {
            if type_name.content() != name {
                continue;
            }
        }

        // Check to see if the type has already been added
        if let Some(added_type) = types.get(type_name.content()) {
            if added_type.ast_id() == Some(ast_id) {
                if name.is_some() {
                    return Ok(Some(added_type.clone()));
                }

                continue;
            }

            return Err(diag.err_span(
                format!(
                    "type name \"{}\" is defined multiple times",
                    type_name.content()
                ),
                type_name.span(),
            ));
        }

        // If not, solve it
        in_progress_types.push(type_name.content());
        let r#type = match item {
            Item::Struct(r#struct) => {
                r#struct::solve(ast, r#struct, in_progress_types, types, diag)?
            }
            Item::Fn(_) => unreachable!(),
        };
        in_progress_types.pop();

        if name.is_some() {
            return Ok(Some(r#type));
        }
    }

    Ok(None)
}

/// Is this item a type definition? If so, returns the type's name and AST ID
fn is_type<'a, 'b>(item: &'b Item<'a>) -> Option<(&'b Identifier<'a>, u32)> {
    match item {
        Item::Struct(r#struct) => Some((r#struct.name(), r#struct.id())),
        Item::Fn(_) => None,
    }
}
