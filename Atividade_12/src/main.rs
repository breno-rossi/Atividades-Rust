use std::fmt::Debug;

trait HasCategory {
    fn category(&self) -> String;
}

#[derive(Debug)]
struct Document {
    name: String,
    doc_type: String,
}

#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
}

impl HasCategory for Document {
    fn category(&self) -> String {
        self.doc_type.clone()
    }
}

impl HasCategory for Task {
    fn category(&self) -> String {
        if self.priority < 5 {
            "High".to_string()
        } else {
            "Low".to_string()
        }
    }
}

fn print_items_by_category<T: HasCategory + Debug>(items: &[T]) {
    println!("--- Processing Items ---");
    for item in items {
        let category = item.category();
        println!("Item: {:?} - Category: {}", item, category);
    }
    println!("--- Done Processing ---");
}

fn main() {
    let documents = vec![
        Document {
            name: "Relatório Anual".to_string(),
            doc_type: "Report".to_string(),
        },
        Document {
            name: "Nota Fiscal #123".to_string(),
            doc_type: "Invoice".to_string(),
        },
        Document {
            name: "Lembrete Reunião".to_string(),
            doc_type: "Memo".to_string(),
        },
    ];

    println!("Processando Documentos:");
    print_items_by_category(&documents);

    let tasks = vec![
        Task {
            description: "Corrigir bug crítico".to_string(),
            priority: 1,
        },
        Task {
            description: "Agendar férias".to_string(),
            priority: 8,
        },
        Task {
            description: "Revisar código".to_string(),
            priority: 4,
        },
    ];

    println!("\nProcessando Tarefas:");
    print_items_by_category(&tasks);
}
