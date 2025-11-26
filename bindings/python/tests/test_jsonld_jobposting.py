"""
Tests for JSON-LD JobPosting type (Schema.org JobPosting)

Following TDD approach - tests written FIRST before implementation
"""

import meta_oxide
import pytest


class TestJobPostingBasic:
    """Test basic JobPosting extraction"""

    def test_jobposting_basic(self):
        """Test minimal job posting with title"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Senior Software Engineer"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert objects[0]["title"] == "Senior Software Engineer"


class TestJobPostingWithOrganization:
    """Test JobPosting with hiring organization"""

    def test_jobposting_with_organization(self):
        """Test job posting with hiringOrganization"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Data Scientist",
                "hiringOrganization": {
                    "@type": "Organization",
                    "name": "Tech Corp",
                    "sameAs": "https://www.techcorp.com",
                    "logo": "https://www.techcorp.com/logo.png"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert objects[0]["title"] == "Data Scientist"
        assert "hiringOrganization" in objects[0]


class TestJobPostingWithLocation:
    """Test JobPosting with job location"""

    def test_jobposting_with_location(self):
        """Test job posting with jobLocation"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Product Manager",
                "jobLocation": {
                    "@type": "Place",
                    "address": {
                        "@type": "PostalAddress",
                        "streetAddress": "555 Clancy St",
                        "addressLocality": "Detroit",
                        "addressRegion": "MI",
                        "postalCode": "48201",
                        "addressCountry": "US"
                    }
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert objects[0]["title"] == "Product Manager"
        assert "jobLocation" in objects[0]

    def test_jobposting_remote(self):
        """Test remote job posting"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Remote Developer",
                "jobLocationType": "TELECOMMUTE",
                "applicantLocationRequirements": {
                    "@type": "Country",
                    "name": "USA"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert "jobLocationType" in objects[0]


class TestJobPostingWithSalary:
    """Test JobPosting with salary information"""

    def test_jobposting_with_salary(self):
        """Test job posting with baseSalary"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Frontend Developer",
                "baseSalary": {
                    "@type": "MonetaryAmount",
                    "currency": "USD",
                    "value": {
                        "@type": "QuantitativeValue",
                        "value": 120000,
                        "unitText": "YEAR"
                    }
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert objects[0]["title"] == "Frontend Developer"
        assert "baseSalary" in objects[0]

    def test_jobposting_with_salary_range(self):
        """Test job posting with salary range"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Backend Engineer",
                "baseSalary": {
                    "@type": "MonetaryAmount",
                    "currency": "USD",
                    "value": {
                        "@type": "QuantitativeValue",
                        "minValue": 100000,
                        "maxValue": 150000,
                        "unitText": "YEAR"
                    }
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert "baseSalary" in objects[0]


class TestJobPostingWithEmploymentType:
    """Test JobPosting with employment type"""

    def test_jobposting_with_employment_type(self):
        """Test job posting with employmentType"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Full Stack Developer",
                "employmentType": ["FULL_TIME", "CONTRACTOR"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert objects[0]["title"] == "Full Stack Developer"
        assert "employmentType" in objects[0]

    def test_jobposting_single_employment_type(self):
        """Test job posting with single employment type"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Intern",
                "employmentType": "INTERN"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert "employmentType" in objects[0]


class TestJobPostingWithDates:
    """Test JobPosting with date information"""

    def test_jobposting_with_dates(self):
        """Test job posting with datePosted and validThrough"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "DevOps Engineer",
                "datePosted": "2024-01-15",
                "validThrough": "2024-03-15T23:59:59Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "JobPosting"
        assert objects[0]["title"] == "DevOps Engineer"
        assert objects[0]["datePosted"] == "2024-01-15"
        assert objects[0]["validThrough"] == "2024-03-15T23:59:59Z"


class TestJobPostingComplete:
    """Test complete JobPosting with all fields"""

    def test_jobposting_complete(self):
        """Test job posting with all supported fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Senior Python Developer",
                "description": "We are seeking an experienced Python developer to join our team.",
                "datePosted": "2024-01-10",
                "validThrough": "2024-04-10T23:59:59Z",
                "employmentType": ["FULL_TIME", "PART_TIME"],
                "hiringOrganization": {
                    "@type": "Organization",
                    "name": "Example Company",
                    "sameAs": "https://example.com",
                    "logo": "https://example.com/logo.png"
                },
                "jobLocation": {
                    "@type": "Place",
                    "address": {
                        "@type": "PostalAddress",
                        "streetAddress": "123 Tech Street",
                        "addressLocality": "San Francisco",
                        "addressRegion": "CA",
                        "postalCode": "94105",
                        "addressCountry": "US"
                    }
                },
                "baseSalary": {
                    "@type": "MonetaryAmount",
                    "currency": "USD",
                    "value": {
                        "@type": "QuantitativeValue",
                        "minValue": 120000,
                        "maxValue": 180000,
                        "unitText": "YEAR"
                    }
                },
                "qualifications": "Bachelor's degree in Computer Science or equivalent experience. 5+ years of Python development.",
                "responsibilities": "Design and develop scalable Python applications. Mentor junior developers. Participate in code reviews.",
                "url": "https://example.com/jobs/senior-python-dev",
                "workHours": "40 hours per week"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        job = objects[0]
        assert job["@type"] == "JobPosting"
        assert job["title"] == "Senior Python Developer"
        assert (
            job["description"] == "We are seeking an experienced Python developer to join our team."
        )
        assert job["datePosted"] == "2024-01-10"
        assert job["validThrough"] == "2024-04-10T23:59:59Z"
        assert "employmentType" in job
        assert "hiringOrganization" in job
        assert "jobLocation" in job
        assert "baseSalary" in job
        assert (
            job["qualifications"]
            == "Bachelor's degree in Computer Science or equivalent experience. 5+ years of Python development."
        )
        assert (
            job["responsibilities"]
            == "Design and develop scalable Python applications. Mentor junior developers. Participate in code reviews."
        )
        assert job["url"] == "https://example.com/jobs/senior-python-dev"
        assert job["workHours"] == "40 hours per week"


class TestJobPostingRealistic:
    """Test realistic job posting examples"""

    def test_tech_job(self):
        """Test realistic tech job posting"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Machine Learning Engineer",
                "description": "Join our AI team to build cutting-edge ML solutions.",
                "identifier": {
                    "@type": "PropertyValue",
                    "name": "Job ID",
                    "value": "ML-2024-001"
                },
                "datePosted": "2024-02-01",
                "validThrough": "2024-05-01T23:59:59Z",
                "employmentType": "FULL_TIME",
                "hiringOrganization": {
                    "@type": "Organization",
                    "name": "AI Innovations Inc",
                    "sameAs": "https://aiinnovations.com"
                },
                "jobLocation": {
                    "@type": "Place",
                    "address": {
                        "@type": "PostalAddress",
                        "addressLocality": "Boston",
                        "addressRegion": "MA",
                        "addressCountry": "US"
                    }
                },
                "baseSalary": {
                    "@type": "MonetaryAmount",
                    "currency": "USD",
                    "value": {
                        "@type": "QuantitativeValue",
                        "value": 150000,
                        "unitText": "YEAR"
                    }
                },
                "qualifications": "MS/PhD in Computer Science, Machine Learning, or related field. Experience with PyTorch or TensorFlow.",
                "responsibilities": "Develop and deploy ML models. Research new algorithms. Collaborate with cross-functional teams.",
                "educationRequirements": {
                    "@type": "EducationalOccupationalCredential",
                    "credentialCategory": "degree"
                },
                "experienceRequirements": {
                    "@type": "OccupationalExperienceRequirements",
                    "monthsOfExperience": 36
                },
                "url": "https://aiinnovations.com/careers/ml-engineer"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        job = objects[0]
        assert job["@type"] == "JobPosting"
        assert job["title"] == "Machine Learning Engineer"
        assert "description" in job
        assert "hiringOrganization" in job
        assert "jobLocation" in job
        assert "baseSalary" in job
        assert "qualifications" in job
        assert "responsibilities" in job
        assert job["url"] == "https://aiinnovations.com/careers/ml-engineer"

    def test_jobposting_with_benefits(self):
        """Test job posting with benefits and additional details"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "UX Designer",
                "description": "Create beautiful and intuitive user experiences.",
                "employmentType": "FULL_TIME",
                "jobBenefits": "Health insurance, 401k, flexible hours, remote work options",
                "industry": "Technology",
                "salaryCurrency": "USD",
                "url": "https://example.com/jobs/ux-designer"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        job = objects[0]
        assert job["@type"] == "JobPosting"
        assert job["title"] == "UX Designer"


class TestJobPostingIntegration:
    """Test JobPosting integration with extract_all()"""

    def test_extract_all_includes_jobposting(self):
        """Test that extract_all() includes JobPosting"""
        html = """
        <html>
        <head>
            <title>Jobs - Senior Engineer</title>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "JobPosting",
                "title": "Senior Engineer",
                "datePosted": "2024-01-15",
                "employmentType": "FULL_TIME"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "JobPosting"
        assert data["jsonld"][0]["title"] == "Senior Engineer"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
