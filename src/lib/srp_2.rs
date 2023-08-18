struct ReportGenerator;

impl ReportGenerator {
    fn generate_report(&self, data: &str) -> String {
        // logic for generating a report
        format!("Generated report: {}", data)
    }
}

struct ReportSaver;

impl ReportSaver {
    fn save_report(&self, report: &str) {
        // logic for saving the report
    }
}

struct ReportEmailSender;

impl ReportEmailSender {
    fn send_report_by_email(&self, report: &str, email: &str) {
        // logic for sending the report by email
    }
}
