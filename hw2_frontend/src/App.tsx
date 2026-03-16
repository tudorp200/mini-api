import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { Container, Row, Col, Card, Button, Form, Navbar, Nav, Badge, ListGroup, Alert } from 'react-bootstrap';

const API_BASE_URL = 'http://localhost:8000/api/v1';

interface Product {
  id: number;
  name: string;
  price: number;
  original_price: number;
  currency: string;
  stock: number;
}

interface BasketItem {
  basket_id: number;
  product_id: number;
  quantity: number;
}

interface ShippingRate {
  provider: string;
  servicelevel: string;
  amount: string;
  currency: string;
  duration: string;
  rate_id: string;
}

function App() {
  const [products, setProducts] = useState<Product[]>([]);
  const [currency, setCurrency] = useState('USD');
  const [basketId, setBasketId] = useState<number | null>(null);
  const [basketItems, setBasketItems] = useState<BasketItem[]>([]);
  const [shippingRates, setShippingRates] = useState<ShippingRate[]>([]);
  const [selectedRateId, setSelectedRateId] = useState('');
  const [address, setAddress] = useState({
    name: 'John Doe',
    street1: '215 Clayton St.',
    city: 'San Francisco',
    state: 'CA',
    zip: '94117',
    country: 'US'
  });
  const [message, setMessage] = useState<{ type: string, text: string } | null>(null);

  useEffect(() => {
    fetchProducts();
    initBasket();
  }, [currency]);

  const fetchProducts = async () => {
    try {
      const resp = await axios.get(`${API_BASE_URL}/products?currency=${currency}`);
      setProducts(resp.data);
    } catch (err) {
      console.error("Failed to fetch products", err);
    }
  };

  const initBasket = async () => {
    let id = localStorage.getItem('basketId');
    if (!id) {
      try {
        const resp = await axios.post(`${API_BASE_URL}/baskets`);
        id = resp.data.id.toString();
        if (id) localStorage.setItem('basketId', id);
      } catch (err) {
        console.error("Failed to create basket", err);
        return;
      }
    }
    setBasketId(Number(id));
    fetchBasketItems(Number(id));
  };

  const fetchBasketItems = async (id: number) => {
    try {
      const resp = await axios.get(`${API_BASE_URL}/baskets/${id}/items`);
      setBasketItems(resp.data);
    } catch (err) {
      console.error("Failed to fetch basket items", err);
    }
  };

  const addToBasket = async (productId: number, quantity: number) => {
    if (!basketId) return;
    try {
      await axios.post(`${API_BASE_URL}/baskets/${basketId}/items`, {
        basket_id: basketId,
        product_id: productId,
        quantity: quantity
      });
      fetchBasketItems(basketId);
      setMessage({ type: 'success', text: 'Item added to basket!' });
    } catch (err) {
      setMessage({ type: 'danger', text: 'Failed to add item to basket.' });
    }
  };

  const deleteBasketItem = async (productId: number) => {
    if (!basketId) return;
    try {
      await axios.delete(`${API_BASE_URL}/baskets/${basketId}/items/${productId}`);
      fetchBasketItems(basketId);
      setMessage({ type: 'success', text: 'Item removed from basket!' });
    } catch (err) {
      setMessage({ type: 'danger', text: 'Failed to remove item from basket.' });
    }
  };

  const updateBasketItemQuantity = async (productId: number, quantity: number) => {
    if (!basketId) return;
    if (quantity === 0) {
      await deleteBasketItem(productId);
      return;
    }
    try {
      await axios.put(`${API_BASE_URL}/baskets/${basketId}/items/${productId}`, {
        basket_id: basketId,
        product_id: productId,
        quantity: quantity
      });
      fetchBasketItems(basketId);
      setMessage({ type: 'success', text: 'Item quantity updated!' });
    } catch (err) {
      setMessage({ type: 'danger', text: 'Failed to update item quantity.' });
    }
  };

  const getShippingQuotes = async () => {
    if (!basketId) return;
    try {
      const resp = await axios.post(`${API_BASE_URL}/shipping/quote?currency=${currency}`, {
        basket_id: basketId,
        to_address: address
      });
      setShippingRates(resp.data.rates);
    } catch (err) {
      setMessage({ type: 'danger', text: 'Failed to get shipping quotes.' });
    }
  };

  const handleCheckout = async () => {
    if (!basketId || !selectedRateId) return;
    try {
      const resp = await axios.post(`${API_BASE_URL}/checkout`, {
        basket_id: basketId,
        currency: currency,
        to_address: address,
        shipping_rate_id: selectedRateId
      });
      setMessage({ type: 'success', text: resp.data.message });
      setBasketItems([]);
      setShippingRates([]);
      localStorage.removeItem('basketId');
      setBasketId(null);
      initBasket();
    } catch (err) {
      setMessage({ type: 'danger', text: 'Checkout failed.' });
    }
  };

  return (
    <div className="App">
      <Navbar bg="dark" variant="dark" expand="lg">
        <Container>
          <Navbar.Brand href="#">HW2 E-Commerce</Navbar.Brand>
          <Nav className="ms-auto">
            <Form.Select 
              value={currency} 
              onChange={(e) => setCurrency(e.target.value)}
              size="sm"
              style={{ width: '100px' }}
            >
              <option value="USD">USD</option>
              <option value="EUR">EUR</option>
              <option value="GBP">GBP</option>
              <option value="RON">RON</option>
            </Form.Select>
            <Nav.Link href="#basket">
              Basket <Badge bg="secondary">{basketItems.reduce((acc, item) => acc + item.quantity, 0)}</Badge>
            </Nav.Link>
          </Nav>
        </Container>
      </Navbar>

      <Container className="mt-4">
        {message && <Alert variant={message.type} onClose={() => setMessage(null)} dismissible>{message.text}</Alert>}
        
        <Row>
          <Col md={8}>
            <h3>Products</h3>
            <Row>
              {products.map(p => (
                <Col key={p.id} md={6} className="mb-3">
                  <Card>
                    <Card.Body>
                      <Card.Title>{p.name}</Card.Title>
                      <Card.Text>
                        Price: {p.price} {p.currency}
                        <br />
                        Stock: {p.stock}
                      </Card.Text>
                      <Form.Group className="mb-2">
                        <Form.Label>Quantity</Form.Label>
                        <Form.Control 
                          type="number" 
                          min="1" 
                          max={p.stock} 
                          defaultValue={1} 
                          onChange={(e) => {
                            // Store quantity in a temporary state or pass directly. For simplicity,
                            // we'll fetch the value when button is clicked or use a ref.
                            // For now, let's keep it simple and assume 1, then update later
                            // if we want per-product quantity state.
                          }}
                          id={`quantity-${p.id}`} // Add an ID for easy access
                        />
                      </Form.Group>
                      <Button variant="primary" onClick={() => {
                        const quantityInput = document.getElementById(`quantity-${p.id}`) as HTMLInputElement;
                        const quantity = quantityInput ? parseInt(quantityInput.value) : 1;
                        addToBasket(p.id, quantity);
                      }} disabled={p.stock <= 0}>
                        Add to Basket
                      </Button>
                    </Card.Body>
                  </Card>
                </Col>
              ))}
            </Row>
          </Col>
          
          <Col md={4}>
            <div id="basket">
              <h3>Your Basket</h3>
              <ListGroup className="mb-3">
                {basketItems.length === 0 ? (
                  <ListGroup.Item>Empty</ListGroup.Item>
                ) : (
                  basketItems.map(item => {
                    const p = products.find(prod => prod.id === item.product_id);
                    return (
                      <ListGroup.Item key={item.product_id} className="d-flex justify-content-between align-items-center">
                        <div>
                          {p?.name || `Product ${item.product_id}`}
                          <Form.Control
                            type="number"
                            min="0"
                            value={item.quantity}
                            onChange={(e) => updateBasketItemQuantity(item.product_id, parseInt(e.target.value))}
                            style={{ width: '80px', display: 'inline-block', marginLeft: '10px' }}
                          />
                        </div>
                        <div>
                          <span>{p ? (p.price * item.quantity).toFixed(2) : ''} {currency}</span>
                          <Button variant="danger" size="sm" className="ms-2" onClick={() => deleteBasketItem(item.product_id)}>
                            X
                          </Button>
                        </div>
                      </ListGroup.Item>
                    );
                  })
                )}
              </ListGroup>

              {basketItems.length > 0 && (
                <>
                  <Card className="mb-3">
                    <Card.Body>
                      <h5>Basket Total</h5>
                      <p className="fw-bold">
                        Total: {
                          basketItems.reduce((acc, item) => {
                            const p = products.find(prod => prod.id === item.product_id);
                            return acc + (p ? p.price * item.quantity : 0);
                          }, 0).toFixed(2)
                        } {currency}
                      </p>
                    </Card.Body>
                  </Card>
                  <Card className="mb-3">
                    <Card.Body>
                      <h5>Shipping Address</h5>
                      <Form.Group className="mb-2">
                        <Form.Control 
                          type="text" placeholder="Name" 
                          value={address.name} 
                          onChange={e => setAddress({...address, name: e.target.value})}
                        />
                      </Form.Group>
                      <Form.Group className="mb-2">
                        <Form.Control 
                          type="text" placeholder="Street" 
                          value={address.street1} 
                          onChange={e => setAddress({...address, street1: e.target.value})}
                        />
                      </Form.Group>
                      <Row>
                        <Col>
                          <Form.Control 
                            type="text" placeholder="City" 
                            value={address.city} 
                            onChange={e => setAddress({...address, city: e.target.value})}
                          />
                        </Col>
                        <Col>
                          <Form.Control 
                            type="text" placeholder="Zip" 
                            value={address.zip} 
                            onChange={e => setAddress({...address, zip: e.target.value})}
                          />
                        </Col>
                      </Row>
                      <Button variant="secondary" className="mt-2 w-100" onClick={getShippingQuotes}>
                        Get Shipping Quotes
                      </Button>
                    </Card.Body>
                  </Card>
                </>
              )}

              {shippingRates.length > 0 && (
                <Card className="mb-3">
                  <Card.Body>
                    <h5>Select Shipping</h5>
                    {shippingRates.map(rate => (
                      <Form.Check 
                        key={rate.rate_id}
                        type="radio"
                        label={`${rate.provider} (${rate.servicelevel}) - ${rate.amount} ${rate.currency}`}
                        name="shippingRate"
                        value={rate.rate_id}
                        onChange={e => setSelectedRateId(e.target.value)}
                      />
                    ))}
                    <Button variant="success" className="mt-3 w-100" onClick={handleCheckout} disabled={!selectedRateId}>
                      Checkout
                    </Button>
                  </Card.Body>
                </Card>
              )}
            </div>
          </Col>
        </Row>
      </Container>
    </div>
  );
}

export default App;