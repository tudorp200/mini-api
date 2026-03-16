import os
import httpx
from typing import Dict
from contextlib import asynccontextmanager
from fastapi import FastAPI, HTTPException, Request
from fastapi.responses import JSONResponse
from pydantic import BaseModel
from dotenv import load_dotenv
import json


env_path = os.path.join(os.path.dirname(__file__), ".env")
load_dotenv(dotenv_path=env_path)

@asynccontextmanager
async def lifespan(app: FastAPI):
    global client
    client = httpx.AsyncClient(timeout=10.0)
    yield

    await client.aclose()

app = FastAPI(
    title="HW2 E-Commerce Aggregator", 
    description="Aggregating HW1 E-commerce with External APIs",
    lifespan=lifespan
)


HW1_API_URL = os.getenv("HW1_API_URL", "http://127.0.0.1:4221")
FASTFOREX_API_KEY = os.getenv("FASTFOREX_API_KEY")
SHIPPO_API_KEY = os.getenv("SHIPPO_API_KEY")

print(f"Loading .env from: {env_path}")
print(f"HW1_API_URL: {HW1_API_URL}")
print(f"FASTFOREX_API_KEY set: {'Yes' if FASTFOREX_API_KEY else 'No'}")
print(f"SHIPPO_API_KEY set: {'Yes' if SHIPPO_API_KEY else 'No'}")

client: httpx.AsyncClient = None


class BasketItem(BaseModel):
    basket_id: int
    product_id: int
    quantity: int

class Product(BaseModel):
    id: int
    category_id: int
    name: str
    price: float
    stock: int
    weight_lb: float = 1.5 

class ShippingRequest(BaseModel):
    basket_id: int
    to_address: Dict[str, str]

class CheckoutRequest(BaseModel):
    basket_id: int
    currency: str = "USD"
    to_address: Dict[str, str]
    shipping_rate_id: str

@app.exception_handler(HTTPException)
async def http_exception_handler(request: Request, exc: HTTPException):
    print(f"HTTP Exception caught: {exc.detail}, Status: {exc.status_code}")
    return JSONResponse(
        status_code=exc.status_code,
        content={"detail": exc.detail},
    )

@app.exception_handler(Exception)
async def global_exception_handler(request: Request, exc: Exception):
    print(f"Global exception caught: {exc}")
    return JSONResponse(
        status_code=500,
        content={"detail": "Internal Server Error", "error": str(exc)},
    )

async def fetch_hw1(path: str, method: str = "GET", json_data: dict = None):
    try:
        url = f"{HW1_API_URL}{path}"
        print(f"Calling HW1: {url}")
        
        if method == "GET":
            resp = await client.get(url)
        else: 

            payload_str = json.dumps(json_data) + "\r\n"
            headers = {
                "Content-Type": "application/json",
                "Content-Length": str(len(payload_str))
            }
            resp = await client.post(url, content=payload_str, headers=headers)
        
        if resp.status_code >= 400:
            raise HTTPException(status_code=resp.status_code, detail=f"HW1 Error: {resp.text}")

        if not resp.text.strip():
            return {"message": "Success"}
            
        return resp.json()
    except httpx.RequestError as e:
        raise HTTPException(status_code=503, detail=f"HW1 unavailable: {str(e)}")

import asyncio
import json

async def force_rust_checkout(basket_id: int):

    payload = json.dumps({
        "id": 0,
        "basket_id": basket_id,
        "total_paid": 0,
        "status": "Not payed"
    })

    http_request = (
        f"POST /orders/checkout HTTP/1.1\r\n"
        f"Host: 127.0.0.1:4221\r\n"
        f"Content-Type: application/json\r\n"
        f"Content-Length: {len(payload)}\r\n"
        f"Connection: close\r\n\r\n"
        f"{payload}"
    )
    
    print(f"Sending raw TCP request to Rust:\n{http_request}")
    
    reader, writer = await asyncio.open_connection('127.0.0.1', 4221)
    writer.write(http_request.encode('utf-8'))
    await writer.drain()
    
    response_data = await reader.read(4096)
    writer.close()
    await writer.wait_closed()
    
    return response_data.decode('utf-8')

async def get_conversion_rate(target: str):
    target = target.upper() 
    if target == "USD":
        print(f"Target currency is USD, no conversion needed.")
        return 1.0
    
    if not FASTFOREX_API_KEY:
        print(f"FASTFOREX_API_KEY is not set. Returning 1.0 for {target}.")
        return 1.0
    
    url = f"https://api.fastforex.io/fetch-one?from=USD&to={target}"

    headers = {"X-API-Key": FASTFOREX_API_KEY}
    
    print(f"Fetching exchange rates from FastForex...")
    try:
        resp = await client.get(url, headers=headers)
        print(f"FastForex response status: {resp.status_code}")
        
        if resp.status_code == 200:
            data = resp.json()
            rate = data.get("result", {}).get(target)
            
            if rate:
                print(f"Found rate for {target}: {rate}")
                return rate
            else:
                print(f"Target currency {target} not found in conversion rates.")
                return 1.0
        else:
            print(f"FastForex non-200 response: {resp.text}")
            return 1.0
            
    except httpx.RequestError as e:
        print(f"FastForex request failed: {str(e)}. Returning 1.0.")
        return 1.0


@app.get("/api/v1/products")
async def list_products(currency: str = "USD"):
    products = await fetch_hw1("/products")
    rate = await get_conversion_rate(currency)
    
    for p in products:
        p["original_price"] = p["price"]
        p["price"] = round(p["price"] * rate, 2)
        p["currency"] = currency.upper()
    return products

@app.post("/api/v1/shipping/quote")
async def get_shipping_quote(req: ShippingRequest):
    items_data = await fetch_hw1(f"/baskets/{req.basket_id}/items")
    items = [BasketItem(**i) for i in items_data]
    
    if not items:
        raise HTTPException(status_code=400, detail="Basket is empty")

    MOCK_WEIGHT_LB = 1.5
    total_weight = sum(item.quantity * MOCK_WEIGHT_LB for item in items) 
    print(f"Calculated total weight for basket {req.basket_id}: {total_weight} lb")

    if not SHIPPO_API_KEY:
        raise HTTPException(status_code=500, detail="Shippo API key missing")

    headers = {"Authorization": f"ShippoToken {SHIPPO_API_KEY}"}
    shippo_payload = {
        "address_to": req.to_address,
        "address_from": {
            "name": "HW2 Warehouse", "street1": "123 Rust Lane", 
            "city": "New York", "state": "NY", "zip": "10115", "country": "US"
        },
        "parcels": [{
            "length": "10", "width": "10", "height": "10", "distance_unit": "cm", 
            "weight": f"{total_weight}", "mass_unit": "lb"
        }],
        "async": False
    }
    
    print(f"Calling Shippo API with payload: {shippo_payload}")
    try:
        resp = await client.post("https://api.goshippo.com/shipments/", headers=headers, json=shippo_payload)
        print(f"Shippo API response status: {resp.status_code}")
        if resp.status_code != 201:
            print(f"Shippo API error response: {resp.text}")
            raise HTTPException(status_code=500, detail=f"Shippo failed: {resp.text}")
        
        rates = resp.json().get("rates", [])
        print(f"Received {len(rates)} shipping rates from Shippo.")
        filtered_rates = [
            {"provider": r.get("provider", "N/A"), "servicelevel": r.get("servicelevel_name", "N/A"), "amount": r.get("amount", "N/A"), "currency": r.get("currency", "N/A"), "duration": r.get("estimated_days", "N/A"), "rate_id": r.get("object_id", "N/A")}
            for r in rates if r.get("amount") and r.get("provider")
        ]
        return {"basket_id": req.basket_id, "total_weight": total_weight, "rates": filtered_rates[:5]}
    except httpx.RequestError as e:
        print(f"Shippo API request failed: {str(e)}. Returning error.")
        raise HTTPException(status_code=503, detail=f"Shippo API unavailable: {str(e)}")

@app.post("/api/v1/checkout")
async def unified_checkout(req: CheckoutRequest):
    print(f"Starting checkout for basket_id: {req.basket_id}")
    
    rust_response = await force_rust_checkout(req.basket_id)
    print(f"HW1 Raw Response: {rust_response}")
    
    if "404" in rust_response or "400" in rust_response:
        raise HTTPException(status_code=400, detail=f"Rust Server Error: {rust_response.splitlines()[-1]}")
    
    rate = await get_conversion_rate(req.currency)
    
    return {
        "message": "Checkout Successful",
        "hw1_response": "Order placed successfully in Rust database!",
        "receipt": {
            "currency": req.currency.upper(),
            "exchange_rate": rate,
            "shipping_status": "Label Pending"
        }
    }
if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
