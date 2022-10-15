pub struct LoginPage;

impl LoginPage {
    fn auth_url(url: &str) {
        let request = ehttp::Request::get(url);
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            
        });
    }
}

impl super::View for LoginPage {
    fn show(ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
        });
    }
}
