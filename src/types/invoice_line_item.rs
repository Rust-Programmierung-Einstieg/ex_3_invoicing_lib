#[derive(Debug)]
pub struct InvoiceLineItem {
    pub name: String,
    pub amount: i64,
    pub net_value: f64,
    pub vat_percentage: f64,
}

impl InvoiceLineItem {
    pub fn vat_amount(self: &Self) -> f64 {
        self.amount as f64 * self.net_value * self.vat_percentage / 100.0
    }

    pub fn total_amount(self: &Self) -> f64 {
        self.amount as f64 * self.net_value * (100.0 + self.vat_percentage) / 100.0
    }
}

#[cfg(test)]
mod tests {
    use crate::types::invoice_line_item::InvoiceLineItem;

    #[test]
    fn invoice_line_item_vat_amount() {
        let ili = InvoiceLineItem {
            name: "test".to_owned(),
            net_value: 10.0,
            vat_percentage: 1.0,
            amount: 2,
        };
        assert_eq!(ili.vat_amount(), 0.2);
    }
    #[test]
    fn invoice_line_item_total_amount() {
        let ili = InvoiceLineItem {
            name: "test".to_owned(),
            net_value: 10.0,
            vat_percentage: 1.0,
            amount: 2,
        };
        assert_eq!(ili.total_amount(), 20.2);
    }
}
