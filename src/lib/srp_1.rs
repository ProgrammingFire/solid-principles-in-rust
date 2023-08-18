struct Report;

impl Report {
    fn generate_report(&self, data: &str) -> String {
        // logic for generating and saving a report
        format!("Generated and saved report: {}", data)
    }

    fn send_report_by_email(&self, report: &str, email: &str) {
        // logic for sending the report by email
    }
}
