use chrono::NaiveDate;

use super::invoice_line_item::InvoiceLineItem;

#[derive(Debug)]
pub struct Invoice {
    number: String,
    date: NaiveDate,
    name: String,
    zip_code: String,
    city: String,
    street: String,
    country: Option<String>,
    items: Vec<InvoiceLineItem>,
}

impl Invoice {
    fn vat_amount(self: &Self) -> f64 {
        self.items
            .iter()
            .map(|i: &InvoiceLineItem| i.vat_amount())
            .sum()
    }

    fn total_amount(self: &Self) -> f64 {
        self.items
            .iter()
            .map(|i: &InvoiceLineItem| i.total_amount())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{invoice::Invoice, invoice_line_item::InvoiceLineItem};

    #[test]
    fn invoice_vat_amount() {
        let invoice = Invoice {
            number: "I0001".to_owned(),
            date: chrono::Utc::now().date_naive(),
            name: "Smallco .inc".to_owned(),
            zip_code: "12345".to_owned(),
            city: "Funtown".to_owned(),
            street: "Somestreet".to_owned(),
            country: None,
            items: vec![],
        };

        let ili = InvoiceLineItem {
            name: "test".to_owned(),
            net_value: 10.0,
            vat_percentage: 1.0,
            amount: 2,
        };
        assert_eq!(ili.vat_amount(), 0.2);
    }
    #[test]
    fn invoice_total_amount() {
        let ili = InvoiceLineItem {
            name: "test".to_owned(),
            net_value: 10.0,
            vat_percentage: 1.0,
            amount: 2,
        };
        assert_eq!(ili.total_amount(), 20.2);
    }
}
