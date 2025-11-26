"""Tests for Dublin Core metadata extraction"""

import meta_oxide


def test_dublin_core_title():
    """Test Dublin Core title extraction"""
    html = '<meta name="DC.title" content="My Document Title">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["title"] == "My Document Title"


def test_dublin_core_creator():
    """Test Dublin Core creator extraction"""
    html = '<meta name="DC.creator" content="John Doe">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["creator"] == "John Doe"


def test_dublin_core_subject():
    """Test Dublin Core subject extraction with multiple values"""
    html = '<meta name="DC.subject" content="rust, metadata, extraction">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["subject"] == ["rust", "metadata", "extraction"]


def test_dublin_core_description():
    """Test Dublin Core description extraction"""
    html = '<meta name="DC.description" content="A comprehensive guide to metadata">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["description"] == "A comprehensive guide to metadata"


def test_dublin_core_publisher():
    """Test Dublin Core publisher extraction"""
    html = '<meta name="DC.publisher" content="Example Press">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["publisher"] == "Example Press"


def test_dublin_core_date():
    """Test Dublin Core date extraction"""
    html = '<meta name="DC.date" content="2024-01-15">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["date"] == "2024-01-15"


def test_dublin_core_type():
    """Test Dublin Core type extraction"""
    html = '<meta name="DC.type" content="Text">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["type"] == "Text"


def test_dublin_core_format():
    """Test Dublin Core format extraction"""
    html = '<meta name="DC.format" content="text/html">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["format"] == "text/html"


def test_dublin_core_identifier():
    """Test Dublin Core identifier extraction"""
    html = '<meta name="DC.identifier" content="ISBN:123-456-789">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["identifier"] == "ISBN:123-456-789"


def test_dublin_core_language():
    """Test Dublin Core language extraction"""
    html = '<meta name="DC.language" content="en-US">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["language"] == "en-US"


def test_dublin_core_rights():
    """Test Dublin Core rights extraction"""
    html = '<meta name="DC.rights" content="Copyright 2024 Example Corp">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["rights"] == "Copyright 2024 Example Corp"


def test_dublin_core_lowercase_prefix():
    """Test Dublin Core with lowercase prefix"""
    html = '<meta name="dc.title" content="Lowercase Prefix">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["title"] == "Lowercase Prefix"


def test_dublin_core_dcterms_prefix():
    """Test Dublin Core with dcterms prefix"""
    html = '<meta name="dcterms.title" content="DCTerms Title">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["title"] == "DCTerms Title"


def test_dublin_core_complete():
    """Test extraction of complete Dublin Core document"""
    html = """
        <html>
        <head>
            <meta name="DC.title" content="Complete Document">
            <meta name="DC.creator" content="Jane Smith">
            <meta name="DC.subject" content="technology, innovation">
            <meta name="DC.description" content="A complete example">
            <meta name="DC.publisher" content="Tech Publishers">
            <meta name="DC.date" content="2024-02-01">
            <meta name="DC.type" content="Article">
            <meta name="DC.format" content="text/html">
            <meta name="DC.identifier" content="DOI:10.1234/example">
            <meta name="DC.language" content="en">
            <meta name="DC.rights" content="CC-BY-4.0">
        </head>
        </html>
    """
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["title"] == "Complete Document"
    assert dc["creator"] == "Jane Smith"
    assert dc["subject"] == ["technology", "innovation"]
    assert dc["description"] == "A complete example"
    assert dc["publisher"] == "Tech Publishers"
    assert dc["date"] == "2024-02-01"
    assert dc["type"] == "Article"
    assert dc["format"] == "text/html"
    assert dc["identifier"] == "DOI:10.1234/example"
    assert dc["language"] == "en"
    assert dc["rights"] == "CC-BY-4.0"


def test_dublin_core_empty():
    """Test empty Dublin Core extraction"""
    html = "<html><head><title>No Dublin Core</title></head></html>"
    dc = meta_oxide.extract_dublin_core(html)
    assert "title" not in dc
    assert "creator" not in dc
    assert "description" not in dc


def test_dublin_core_contributor_list():
    """Test Dublin Core contributor with multiple values"""
    html = '<meta name="DC.contributor" content="Alice, Bob, Charlie">'
    dc = meta_oxide.extract_dublin_core(html)
    assert dc["contributor"] == ["Alice", "Bob", "Charlie"]
