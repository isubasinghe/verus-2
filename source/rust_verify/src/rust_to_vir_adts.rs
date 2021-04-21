use crate::rust_to_vir_expr::{hack_get_def_name, spanned_new, ty_to_vir};
use crate::rust_to_vir_func::check_generics;
use crate::{unsupported, unsupported_unless};
use rustc_hir::{Crate, EnumDef, Generics, ItemId, VariantData};
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;
use std::rc::Rc;
use vir::ast::{Ident, KrateX, Variant};
use vir::ast_util::{ident_binder, str_ident};

// TODO(andreal): move to adt file
fn check_variant_data<'tcx>(
    tcx: TyCtxt<'tcx>,
    _krate: &'tcx Crate<'tcx>,
    name: &Ident,
    variant_data: &'tcx VariantData<'tcx>,
) -> Variant {
    ident_binder(
        name,
        &(match variant_data {
            VariantData::Struct(fields, recovered) => {
                unsupported_unless!(!recovered, "recovered_struct", variant_data);
                Rc::new(
                    fields
                        .iter()
                        .map(|field| {
                            ident_binder(
                                &str_ident(&field.ident.as_str()),
                                &ty_to_vir(tcx, field.ty),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            }
            VariantData::Tuple(fields, _variant_id) => Rc::new(
                fields
                    .iter()
                    .map(|field| {
                        ident_binder(&str_ident(&field.ident.as_str()), &ty_to_vir(tcx, field.ty))
                    })
                    .collect::<Vec<_>>(),
            ),
            VariantData::Unit(_) => {
                unsupported!("unit_adt", variant_data);
            }
        }),
    )
}

// TODO(andreal): move to adt file
pub fn check_item_struct<'tcx>(
    tcx: TyCtxt<'tcx>,
    krate: &'tcx Crate<'tcx>,
    vir: &mut KrateX,
    span: Span,
    id: &ItemId,
    variant_data: &'tcx VariantData<'tcx>,
    generics: &'tcx Generics<'tcx>,
) {
    check_generics(generics);
    let name = hack_get_def_name(tcx, id.def_id.to_def_id());
    let variant_name = str_ident(format!("{}::{}", name, name).as_str());
    let variants = Rc::new(vec![check_variant_data(tcx, krate, &variant_name, variant_data)]);
    vir.datatypes.push(spanned_new(span, ident_binder(&Rc::new(name), &variants)));
}

// TODO(andreal): move to adt file
pub fn check_item_enum<'tcx>(
    tcx: TyCtxt<'tcx>,
    krate: &'tcx Crate<'tcx>,
    vir: &mut KrateX,
    span: Span,
    id: &ItemId,
    enum_def: &'tcx EnumDef<'tcx>,
    generics: &'tcx Generics<'tcx>,
) {
    check_generics(generics);
    let name = Rc::new(hack_get_def_name(tcx, id.def_id.to_def_id()));
    let variants = Rc::new(
        enum_def
            .variants
            .iter()
            .map(|variant| {
                let rust_variant_name = variant.ident.as_str();
                let variant_name = str_ident(format!("{}::{}", name, rust_variant_name).as_str());
                check_variant_data(tcx, krate, &variant_name, &variant.data)
            })
            .collect::<Vec<_>>(),
    );
    vir.datatypes.push(spanned_new(span, ident_binder(&name, &variants)));
}