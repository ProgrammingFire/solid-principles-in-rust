trait Payment {
    fn process(&self);
}

struct CreditCardPayment;

impl Payment for CreditCardPayment {
    fn process(&self) {
        // logic for credit card payment processing
    }
}

struct PayPalPayment;

impl Payment for PayPalPayment {
    fn process(&self) {
        // logic for PayPal payment processing
    }
}

// Adding a new payment method without modifying existing code
struct BitcoinPayment;

impl Payment for BitcoinPayment {
    fn process(&self) {
        // logic for Bitcoin payment processing
    }
}

struct PayTMPayment;

impl Payment for PayTMPayment {
    fn process(&self) {
        // logic
    }
}
