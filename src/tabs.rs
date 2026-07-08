/// Enum para representar as abas da aplicação
/// Isto torna o código mais seguro e legível do que usar índices numéricos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard = 0,
    Explorer = 1,
    PayloadGen = 2,
    Sessions = 3,
    Console = 4,
    Resources = 5,
}

impl Tab {
    /// Converte de índice para Tab
    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Tab::Dashboard),
            1 => Some(Tab::Explorer),
            2 => Some(Tab::PayloadGen),
            3 => Some(Tab::Sessions),
            4 => Some(Tab::Console),
            5 => Some(Tab::Resources),
            _ => None,
        }
    }

    /// Converte Tab para índice
    pub fn as_index(self) -> usize {
        self as usize
    }

    /// Próxima aba (circular)
    pub fn next(self) -> Self {
        match self {
            Tab::Dashboard => Tab::Explorer,
            Tab::Explorer => Tab::PayloadGen,
            Tab::PayloadGen => Tab::Sessions,
            Tab::Sessions => Tab::Console,
            Tab::Console => Tab::Resources,
            Tab::Resources => Tab::Dashboard,
        }
    }

    /// Aba anterior (circular)
    pub fn previous(self) -> Self {
        match self {
            Tab::Dashboard => Tab::Resources,
            Tab::Resources => Tab::Console,
            Tab::Console => Tab::Sessions,
            Tab::Sessions => Tab::PayloadGen,
            Tab::PayloadGen => Tab::Explorer,
            Tab::Explorer => Tab::Dashboard,
        }
    }

    /// Nome para exibição
    pub fn label(&self) -> &'static str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Explorer => "Explorer",
            Tab::PayloadGen => "Payload",
            Tab::Sessions => "Sessions",
            Tab::Console => "Console",
            Tab::Resources => "Scripts",
        }
    }

    /// Número do Tab para navegação por teclado (1-6)
    pub fn key_number(&self) -> char {
        ((self.as_index() + 1) as u8 + b'0') as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_from_index() {
        assert_eq!(Tab::from_index(0), Some(Tab::Dashboard));
        assert_eq!(Tab::from_index(3), Some(Tab::Sessions));
        assert_eq!(Tab::from_index(5), Some(Tab::Resources));
        assert_eq!(Tab::from_index(6), None);
    }

    #[test]
    fn test_tab_as_index() {
        assert_eq!(Tab::Dashboard.as_index(), 0);
        assert_eq!(Tab::Sessions.as_index(), 3);
        assert_eq!(Tab::Resources.as_index(), 5);
    }

    #[test]
    fn test_tab_navigation() {
        assert_eq!(Tab::Dashboard.next(), Tab::Explorer);
        assert_eq!(Tab::Resources.next(), Tab::Dashboard);
        assert_eq!(Tab::Dashboard.previous(), Tab::Resources);
        assert_eq!(Tab::Explorer.previous(), Tab::Dashboard);
    }
}
