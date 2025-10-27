pub trait Scalar<G: Group>: Clone + PartialEq + std::fmt::Debug {

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn neg(&self) -> Self; // Additive inverse
    fn inv(&self) -> Self; // multiplicative inverse

    fn to_bytes(&self) -> Vec<u8>;

    fn group(&self) -> G;
}

pub trait Element<G: Group>: Clone + PartialEq + std::fmt::Debug {

    /// operacao generica do grupo
    fn add(&self, other: &Self) -> Self;

    /// aplicacao repetida de `add`
    fn mul_scalar(&self, scalar: &G::Scalar) -> Self;

    /// inverso do elemento no grupo
    fn inv(&self) -> Self;

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.inv())
    }

    fn to_bytes(&self) -> Vec<u8>;

    fn group(&self) -> G;
}

pub trait Group: Clone + PartialEq + std::fmt::Debug {
    type Scalar: Scalar<Self>;
    type Element: Element<Self>;

    /// retorna o elemento identidade do grupo
    fn identity(&self) -> Self::Element;

    fn zero(&self) -> Self::Scalar;

    fn one(&self) -> Self::Scalar;

    /// retorna um elemento aleatorio pertencente ao grupo
    fn random_element(&self) -> Self::Element;

    /// retorna um escalar aleatorio em [0, q)
    fn random_scalar(&self) -> Self::Scalar;

    /// multiplica o gerador do grupo pelo escalar `scalar`
    fn mul_generator(&self, scalar: &Self::Scalar) -> Self::Element;

    fn element_from_bytes(&self, bytes: &[u8]) -> Self::Element;

    fn scalar_from_bytes(&self, bytes: &[u8]) -> Self::Scalar;
}