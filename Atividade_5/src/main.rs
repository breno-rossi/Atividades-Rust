struct Sala {
    largura: f64,
    profundidade: f64,
    altura : f64,
}

impl Sala{
    fn area_piso(&self) -> f64 {
        self.largura * self.profundidade
    }

    fn area_parede_frontal(&self) -> f64{
        self.profundidade * self.altura
    }

    fn area_parede_lateral(&self) -> f64{
        self.largura * self.altura
    }

    fn volume (&self) -> f64{
        self.profundidade * self.altura * self.largura
    }

    fn soma_volume (&self, v2: &Sala) -> f64{
        self.volume() + v2.volume()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_piso() {
        let sala = Sala {largura: 2.0, profundidade: 1.0, altura: 4.0};
        assert_eq!(sala.area_piso(), 2.0);
    }

    #[test]
    fn test_area_parede_frontal() {
        let sala = Sala {largura: 2.0, profundidade: 1.0, altura: 4.0};
        assert_eq!(sala.area_parede_frontal(), 4.0);
    }

    #[test]
    fn test_area_parede_lateral() {
        let sala = Sala {largura: 2.0, profundidade: 1.0, altura: 4.0};
        assert_eq!(sala.area_parede_lateral(), 8.0);
    }

    #[test]
    fn test_volume() {
        let sala = Sala {largura: 2.0, profundidade: 1.0, altura: 4.0};
        assert_eq!(sala.volume(), 8.0);
    }

    #[test]
    fn test_soma_volume() {
        let sala1 = Sala {largura: 2.0, profundidade: 1.0, altura: 4.0};
        let sala2 = Sala {largura: 3.0, profundidade: 2.0, altura: 5.0};
        assert_eq!(sala1.soma_volume(&sala2), 38.0);
    }

}
