"""Tests for h-geo microformat extraction"""

import meta_oxide


def test_extract_hgeo_basic():
    """Test basic h-geo extraction"""
    html = """
        <div class="h-geo">
            <span class="p-latitude">37.7749</span>
            <span class="p-longitude">-122.4194</span>
        </div>
    """
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 1
    assert abs(geos[0]["latitude"] - 37.7749) < 0.0001
    assert abs(geos[0]["longitude"] - (-122.4194)) < 0.0001


def test_hgeo_with_altitude():
    """Test h-geo with altitude"""
    html = """
        <div class="h-geo">
            <span class="p-latitude">40.7128</span>
            <span class="p-longitude">-74.0060</span>
            <span class="p-altitude">10</span>
        </div>
    """
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 1
    assert abs(geos[0]["latitude"] - 40.7128) < 0.0001
    assert abs(geos[0]["longitude"] - (-74.0060)) < 0.0001
    assert abs(geos[0]["altitude"] - 10.0) < 0.0001


def test_hgeo_san_francisco():
    """Test San Francisco coordinates"""
    html = """
        <div class="h-geo">
            <span class="p-latitude">37.7749</span>
            <span class="p-longitude">-122.4194</span>
        </div>
    """
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 1
    # San Francisco coordinates
    assert 37.7 < geos[0]["latitude"] < 37.8
    assert -122.5 < geos[0]["longitude"] < -122.4


def test_hgeo_mount_everest():
    """Test Mount Everest coordinates with altitude"""
    html = """
        <div class="h-geo">
            <span class="p-latitude">27.9881</span>
            <span class="p-longitude">86.9250</span>
            <span class="p-altitude">8848</span>
        </div>
    """
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 1
    assert abs(geos[0]["latitude"] - 27.9881) < 0.0001
    assert abs(geos[0]["longitude"] - 86.9250) < 0.0001
    assert abs(geos[0]["altitude"] - 8848.0) < 0.1


def test_hgeo_negative_altitude():
    """Test location with negative altitude (below sea level)"""
    html = """
        <div class="h-geo">
            <span class="p-latitude">31.5590</span>
            <span class="p-longitude">35.4732</span>
            <span class="p-altitude">-430.5</span>
        </div>
    """
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 1
    assert abs(geos[0]["altitude"] - (-430.5)) < 0.1


def test_multiple_hgeo():
    """Test extraction of multiple geo locations"""
    html = """
        <div class="h-geo">
            <span class="p-latitude">51.5074</span>
            <span class="p-longitude">-0.1278</span>
        </div>
        <div class="h-geo">
            <span class="p-latitude">48.8566</span>
            <span class="p-longitude">2.3522</span>
        </div>
    """
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 2
    # London
    assert abs(geos[0]["latitude"] - 51.5074) < 0.0001
    # Paris
    assert abs(geos[1]["latitude"] - 48.8566) < 0.0001


def test_hgeo_empty():
    """Test page with no geo data"""
    html = "<html><body><p>No geo data here</p></body></html>"
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 0


def test_hgeo_equator_prime_meridian():
    """Test coordinates at equator and prime meridian"""
    html = """
        <div class="h-geo">
            <span class="p-latitude">0.0</span>
            <span class="p-longitude">0.0</span>
        </div>
    """
    geos = meta_oxide.extract_hgeo(html)
    assert len(geos) == 1
    assert abs(geos[0]["latitude"]) < 0.0001
    assert abs(geos[0]["longitude"]) < 0.0001
