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

    fn soma_volumes (&self, v2: &Sala) -> f64{
        self.volume() + v2.volume()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testes com dimensões positivas e não nulas
    #[test]
    fn test_area_piso() {
        let sala = Sala { largura: 2.0, profundidade: 1.0, altura: 4.0 };
        assert_eq!(sala.area_piso(), 2.0);

        // Teste adicional
        let sala2 = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala2.area_piso(), 20.0);
    }

    #[test]
    fn test_area_parede_frontal() {
        let sala = Sala { largura: 2.0, profundidade: 1.0, altura: 4.0 };
        assert_eq!(sala.area_parede_frontal(), 4.0);

        // Teste adicional
        let sala2 = Sala { largura: 5.0, profundidade: 3.0, altura: 2.5 };
        assert_eq!(sala2.area_parede_frontal(), 7.5);
    }

    #[test]
    fn test_area_parede_lateral() {
        let sala = Sala { largura: 2.0, profundidade: 1.0, altura: 4.0 };
        assert_eq!(sala.area_parede_lateral(), 8.0);

        // Teste adicional
        let sala2 = Sala { largura: 6.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala2.area_parede_lateral(), 18.0);
    }

    #[test]
    fn test_volume() {
        let sala = Sala { largura: 2.0, profundidade: 1.0, altura: 4.0 };
        assert_eq!(sala.volume(), 8.0);

        // Teste adicional
        let sala2 = Sala { largura: 3.0, profundidade: 2.0, altura: 2.5 };
        assert_eq!(sala2.volume(), 15.0);
    }

    #[test]
    fn test_soma_volumes() {
        let sala1 = Sala { largura: 2.0, profundidade: 1.0, altura: 4.0 };
        let sala2 = Sala { largura: 3.0, profundidade: 2.0, altura: 5.0 };
        assert_eq!(sala1.soma_volumes(&sala2), 38.0);

        // Teste do exemplo dos requisitos
        let sala3 = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        let sala4 = Sala { largura: 3.0, profundidade: 2.0, altura: 2.5 };
        assert_eq!(sala3.soma_volumes(&sala4), 75.0); // 60.0 + 15.0
    }

    // Testes com dimensões zero (casos extremos)
    #[test]
    fn test_area_piso_com_zero() {
        let sala = Sala { largura: 0.0, profundidade: 5.0, altura: 3.0 };
        assert_eq!(sala.area_piso(), 0.0);

        let sala2 = Sala { largura: 4.0, profundidade: 0.0, altura: 3.0 };
        assert_eq!(sala2.area_piso(), 0.0);
    }

    #[test]
    fn test_area_parede_frontal_com_zero() {
        let sala = Sala { largura: 5.0, profundidade: 0.0, altura: 3.0 };
        assert_eq!(sala.area_parede_frontal(), 0.0);

        let sala2 = Sala { largura: 5.0, profundidade: 4.0, altura: 0.0 };
        assert_eq!(sala2.area_parede_frontal(), 0.0);
    }

    #[test]
    fn test_area_parede_lateral_com_zero() {
        let sala = Sala { largura: 0.0, profundidade: 5.0, altura: 3.0 };
        assert_eq!(sala.area_parede_lateral(), 0.0);

        let sala2 = Sala { largura: 4.0, profundidade: 5.0, altura: 0.0 };
        assert_eq!(sala2.area_parede_lateral(), 0.0);
    }

    #[test]
    fn test_volume_com_zero() {
        let sala = Sala { largura: 0.0, profundidade: 5.0, altura: 3.0 };
        assert_eq!(sala.volume(), 0.0);

        let sala2 = Sala { largura: 4.0, profundidade: 0.0, altura: 3.0 };
        assert_eq!(sala2.volume(), 0.0);

        let sala3 = Sala { largura: 4.0, profundidade: 5.0, altura: 0.0 };
        assert_eq!(sala3.volume(), 0.0);
    }

    #[test]
    fn test_soma_volumes_com_zero() {
        let sala1 = Sala { largura: 0.0, profundidade: 0.0, altura: 0.0 };
        let sala2 = Sala { largura: 3.0, profundidade: 2.0, altura: 5.0 };
        assert_eq!(sala1.soma_volumes(&sala2), 30.0); // 0.0 + 30.0

        let sala3 = Sala { largura: 0.0, profundidade: 0.0, altura: 0.0 };
        let sala4 = Sala { largura: 0.0, profundidade: 0.0, altura: 0.0 };
        assert_eq!(sala3.soma_volumes(&sala4), 0.0);
    }
}
