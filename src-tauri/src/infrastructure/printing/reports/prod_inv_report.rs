use crate::common::error::AppError;
use crate::domain::report_models::product_inventory::ProductInventoryReport;
use crate::domain::report_models::product_inventory::ProductInventoryTotals;
use crate::infrastructure::printing::paginator::Paginator;
use crate::infrastructure::printing::print::print_pdf_silently;
use crate::infrastructure::printing::reports::common::account_footer;
use crate::infrastructure::printing::reports::common::util::format_cents;
use printpdf::{BuiltinFont, Mm, PdfDocument, PdfLayerReference};
use std::io::Write;

// Prints the product inventory report PDF and sends to printer.
pub fn print_inventory_report(
    rows: &[ProductInventoryReport],
    product_totals: ProductInventoryTotals,
    total_amount: i32,
    printer_name: &str,
) -> Result<(), AppError> {
    // layout constants
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_top = Mm(25.0);
    let margin_bottom = Mm(15.0);
    let line_height = Mm(7.0);
    let footer_height = Mm(10.0 + 4.0);

    let (doc, first_page, first_layer) =
        PdfDocument::new("Inventory Report", page_width, page_height, "Layer1");

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // closures for header & footer
    let draw_header = |layer: &PdfLayerReference| {
        let y = page_height - margin_top;
        layer.use_text("Qty", 11.0, Mm(25.0), y, &bold);
        layer.use_text("Name", 11.0, Mm(40.0), y, &bold);
        layer.use_text("UPC", 11.0, Mm(120.0), y, &bold);
        layer.use_text("Total", 11.0, Mm(170.0), y, &bold);
    };
    let draw_footer = |layer: &PdfLayerReference| {
        account_footer::account_footer(layer, &font, &bold, total_amount);
    };

    // replace the entire pagination block with Paginator
    {
        let mut pg = Paginator::new(
            &doc,
            first_page,
            first_layer,
            page_width,
            page_height,
            margin_top,
            margin_bottom,
            line_height,
            footer_height,
            draw_header,
            draw_footer,
        );
        let mut last_category: Option<&str> = None;

        for (i, r) in rows.iter().enumerate() {
            // how much space we'll need
            let extra = if !r.is_summary && (last_category != Some(r.category.as_str())) {
                Mm(16.0)
            } else {
                Mm(0.0)
            };

            if !r.is_summary && (last_category != Some(r.category.as_str())) {
                let layer = pg.layer_for(extra + line_height);
                pg.advance(Mm(8.0));
                layer.use_text(&r.category, 12.0, Mm(40.0), pg.current_y(), &bold);
                pg.advance(Mm(8.0));
                last_category = Some(r.category.as_str());
            }

            let layer = pg.layer_for(line_height);

            // draw the row
            if r.is_summary {
                layer.use_text(r.quantity.to_string(), 9.0, Mm(25.0), pg.current_y(), &bold);
                layer.use_text(format_cents(r.total), 9.0, Mm(170.0), pg.current_y(), &bold);
            } else {
                let txt = r.upc.as_deref().unwrap_or("---");
                layer.use_text(r.quantity.to_string(), 9.0, Mm(25.0), pg.current_y(), &font);
                layer.use_text(
                    r.name.as_deref().unwrap_or("---"),
                    9.0,
                    Mm(40.0),
                    pg.current_y(),
                    &font,
                );
                layer.use_text(txt, 9.0, Mm(120.0), pg.current_y(), &font);
                layer.use_text(format_cents(r.total), 9.0, Mm(170.0), pg.current_y(), &font);
            }
            if i < rows.len() - 1 {
                pg.advance(line_height);
            }
        }

        pg.advance(Mm(2.0));

        let layer = pg.layer_for(Mm(7.0));

        // draw separator line with underscores
        layer.use_text("____", 9.0, Mm(25.0), pg.current_y(), &font); // Qty column
        layer.use_text("________", 9.0, Mm(170.0), pg.current_y(), &font); // Total column

        pg.advance(Mm(5.0));
        let layer = pg.layer_for(line_height);

        // draw grand totals
        layer.use_text(
            product_totals.total_quantity.to_string(),
            9.0,
            Mm(25.0),
            pg.current_y(),
            &bold,
        );
        layer.use_text(
            format_cents(product_totals.total_value),
            9.0,
            Mm(170.0),
            pg.current_y(),
            &bold,
        );

        // signature block
        let sig_lines = Mm(line_height.0 * 11.0);
        let layer = pg.layer_for(sig_lines);
        pg.advance(line_height * 6.0);
        layer.use_text(
            "Resident Signature:____________________________________________",
            9.0,
            Mm(40.0),
            pg.current_y(),
            &bold,
        );
        pg.advance(line_height);
        pg.advance(line_height * 3.0);
        layer.use_text(
            "Staff Signature:_______________________________________________",
            9.0,
            Mm(40.0),
            pg.current_y(),
            &bold,
        );

        // finish the last page footer
        pg.finalize();
        pg.draw_page_numbers(&font);
    }

    // save & dispatch print
    let path = "inventory_report.pdf";
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    doc.save(&mut file)?;
    file.flush()?;

    let printer = printer_name.to_string();
    let pdf_path = path.to_string();
    std::thread::spawn(move || {
        if let Err(e) = print_pdf_silently(&pdf_path, &printer) {
            log::error!("Failed to print PDF silently: {e}");
        }
    });

    Ok(())
}
