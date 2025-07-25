use crate::application::use_cases::printer_usecases::PrinterUseCases;
use crate::common::error::AppError;
use crate::interface::presenters::printer_presenter::PrinterPresenter;
#[derive(Debug)]
enum ReportType {
    Receipt,
}

impl std::str::FromStr for ReportType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "receipt" => Ok(ReportType::Receipt),
            _ => Err(AppError::NotFound(format!("Unknown report: {}", s))),
        }
    }
}

pub struct PrinterController {
    uc: PrinterUseCases,
}

impl PrinterController {
    #[must_use]
    pub const fn new(uc: PrinterUseCases) -> Self {
        Self { uc }
    }

    // Returns a list of printers as DTOs.
    pub fn list_printers(
        &self,
    ) -> Result<Vec<crate::interface::dto::printer_dto::PrinterDto>, AppError> {
        let names = self.uc.list_printers()?;
        Ok(PrinterPresenter::to_dto(names))
    }
}
