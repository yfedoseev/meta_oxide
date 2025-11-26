"""Tests for h-adr microformat extraction"""

import meta_oxide


def test_extract_hadr_basic():
    """Test basic h-adr extraction"""
    html = """
        <div class="h-adr">
            <span class="p-street-address">123 Main St</span>
            <span class="p-locality">San Francisco</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    assert addresses[0]["street_address"] == "123 Main St"
    assert addresses[0]["locality"] == "San Francisco"


def test_hadr_complete():
    """Test h-adr with all properties"""
    html = """
        <div class="h-adr">
            <span class="p-street-address">123 Main Street</span>
            <span class="p-extended-address">Suite 400</span>
            <span class="p-locality">San Francisco</span>
            <span class="p-region">CA</span>
            <span class="p-postal-code">94102</span>
            <span class="p-country-name">USA</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    addr = addresses[0]
    assert addr["street_address"] == "123 Main Street"
    assert addr["extended_address"] == "Suite 400"
    assert addr["locality"] == "San Francisco"
    assert addr["region"] == "CA"
    assert addr["postal_code"] == "94102"
    assert addr["country_name"] == "USA"


def test_hadr_with_po_box():
    """Test h-adr with PO Box"""
    html = """
        <div class="h-adr">
            <span class="p-post-office-box">PO Box 123</span>
            <span class="p-locality">Springfield</span>
            <span class="p-postal-code">12345</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    assert addresses[0]["post_office_box"] == "PO Box 123"
    assert addresses[0]["locality"] == "Springfield"


def test_hadr_international():
    """Test international address"""
    html = """
        <div class="h-adr">
            <span class="p-street-address">10 Downing Street</span>
            <span class="p-locality">London</span>
            <span class="p-postal-code">SW1A 2AA</span>
            <span class="p-country-name">United Kingdom</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    addr = addresses[0]
    assert addr["street_address"] == "10 Downing Street"
    assert addr["locality"] == "London"
    assert addr["postal_code"] == "SW1A 2AA"
    assert addr["country_name"] == "United Kingdom"


def test_hadr_us_format():
    """Test US-style address"""
    html = """
        <div class="h-adr">
            <span class="p-street-address">1600 Pennsylvania Avenue NW</span>
            <span class="p-locality">Washington</span>
            <span class="p-region">DC</span>
            <span class="p-postal-code">20500</span>
            <span class="p-country-name">United States</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    addr = addresses[0]
    assert addr["street_address"] == "1600 Pennsylvania Avenue NW"
    assert addr["locality"] == "Washington"
    assert addr["region"] == "DC"


def test_hadr_canada():
    """Test Canadian address"""
    html = """
        <div class="h-adr">
            <span class="p-street-address">24 Sussex Drive</span>
            <span class="p-locality">Ottawa</span>
            <span class="p-region">Ontario</span>
            <span class="p-postal-code">K1M 1M4</span>
            <span class="p-country-name">Canada</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    addr = addresses[0]
    assert addr["locality"] == "Ottawa"
    assert addr["region"] == "Ontario"
    assert addr["country_name"] == "Canada"


def test_multiple_hadr():
    """Test extraction of multiple addresses"""
    html = """
        <div class="h-adr">
            <span class="p-locality">New York</span>
        </div>
        <div class="h-adr">
            <span class="p-locality">Los Angeles</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 2
    assert addresses[0]["locality"] == "New York"
    assert addresses[1]["locality"] == "Los Angeles"


def test_hadr_empty():
    """Test page with no addresses"""
    html = "<html><body><p>No addresses here</p></body></html>"
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 0


def test_hadr_minimal():
    """Test h-adr with minimal properties"""
    html = """
        <div class="h-adr">
            <span class="p-locality">Seattle</span>
            <span class="p-region">WA</span>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    assert addresses[0]["locality"] == "Seattle"
    assert addresses[0]["region"] == "WA"


def test_hadr_nested_in_hcard():
    """Test h-adr nested in h-card"""
    html = """
        <div class="h-card">
            <span class="p-name">John Doe</span>
            <div class="p-adr h-adr">
                <span class="p-street-address">456 Elm St</span>
                <span class="p-locality">Portland</span>
                <span class="p-region">OR</span>
            </div>
        </div>
    """
    addresses = meta_oxide.extract_hadr(html)
    assert len(addresses) == 1
    assert addresses[0]["street_address"] == "456 Elm St"
    assert addresses[0]["locality"] == "Portland"
