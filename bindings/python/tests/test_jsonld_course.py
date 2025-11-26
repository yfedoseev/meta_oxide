"""
Tests for JSON-LD Course type (Schema.org Course)

Following TDD approach - tests written FIRST before implementation
"""

import meta_oxide
import pytest


class TestCourseBasic:
    """Test basic Course extraction"""

    def test_course_basic(self):
        """Test minimal course with name"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Introduction to Python Programming"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert objects[0]["name"] == "Introduction to Python Programming"


class TestCourseWithProvider:
    """Test Course with provider organization"""

    def test_course_with_provider(self):
        """Test course with provider organization"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Data Science Fundamentals",
                "description": "Learn data science basics",
                "provider": {
                    "@type": "Organization",
                    "name": "Tech University",
                    "url": "https://techuniversity.edu"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert objects[0]["name"] == "Data Science Fundamentals"
        assert objects[0]["description"] == "Learn data science basics"
        assert "provider" in objects[0]


class TestCourseWithPrerequisites:
    """Test Course with prerequisites"""

    def test_course_with_prerequisites(self):
        """Test course with prerequisites array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Advanced Machine Learning",
                "description": "Deep dive into ML algorithms",
                "coursePrerequisites": [
                    "Introduction to Python",
                    "Statistics 101",
                    "Linear Algebra"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert objects[0]["name"] == "Advanced Machine Learning"
        assert "coursePrerequisites" in objects[0]
        assert isinstance(objects[0]["coursePrerequisites"], list)
        assert len(objects[0]["coursePrerequisites"]) == 3

    def test_course_with_teaches(self):
        """Test course with teaches field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Web Development Bootcamp",
                "teaches": [
                    "HTML",
                    "CSS",
                    "JavaScript",
                    "React"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert "teaches" in objects[0]
        assert isinstance(objects[0]["teaches"], list)
        assert len(objects[0]["teaches"]) == 4


class TestCourseWithInstances:
    """Test Course with course instances"""

    def test_course_with_instances(self):
        """Test course with hasCourseInstance field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Introduction to AI",
                "hasCourseInstance": [
                    {
                        "@type": "CourseInstance",
                        "courseMode": "online",
                        "startDate": "2024-09-01",
                        "endDate": "2024-12-15",
                        "instructor": {
                            "@type": "Person",
                            "name": "Dr. Jane Smith"
                        }
                    },
                    {
                        "@type": "CourseInstance",
                        "courseMode": "onsite",
                        "startDate": "2024-10-01",
                        "endDate": "2024-12-20"
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert objects[0]["name"] == "Introduction to AI"
        assert "hasCourseInstance" in objects[0]
        assert isinstance(objects[0]["hasCourseInstance"], list)
        assert len(objects[0]["hasCourseInstance"]) == 2


class TestCourseWithRating:
    """Test Course with aggregate rating"""

    def test_course_with_rating(self):
        """Test course with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Full Stack Development",
                "description": "Become a full stack developer",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.8",
                    "reviewCount": "325",
                    "bestRating": "5",
                    "worstRating": "1"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert objects[0]["name"] == "Full Stack Development"
        assert "aggregateRating" in objects[0]


class TestCourseWithOffers:
    """Test Course with pricing/enrollment offers"""

    def test_course_with_offers(self):
        """Test course with offers for pricing information"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Cloud Computing Essentials",
                "description": "Master cloud technologies",
                "offers": {
                    "@type": "Offer",
                    "price": "299.99",
                    "priceCurrency": "USD",
                    "availability": "https://schema.org/InStock",
                    "url": "https://example.com/enroll/cloud-computing"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert objects[0]["name"] == "Cloud Computing Essentials"
        assert "offers" in objects[0]

    def test_course_with_multiple_offers(self):
        """Test course with array of offers"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Digital Marketing",
                "offers": [
                    {
                        "@type": "Offer",
                        "price": "199.99",
                        "priceCurrency": "USD",
                        "name": "Basic Plan"
                    },
                    {
                        "@type": "Offer",
                        "price": "399.99",
                        "priceCurrency": "USD",
                        "name": "Premium Plan"
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert "offers" in objects[0]


class TestCourseComplete:
    """Test complete course with all fields"""

    def test_course_complete(self):
        """Test course with all supported fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Complete Web Development Bootcamp",
                "description": "From zero to full-stack developer in 12 weeks",
                "url": "https://example.com/courses/web-dev-bootcamp",
                "image": "https://example.com/images/web-dev-course.jpg",
                "provider": {
                    "@type": "Organization",
                    "name": "CodeAcademy Pro",
                    "url": "https://example.com"
                },
                "hasCourseInstance": [
                    {
                        "@type": "CourseInstance",
                        "courseMode": "online",
                        "startDate": "2024-09-15",
                        "endDate": "2024-12-15"
                    }
                ],
                "teaches": [
                    "HTML/CSS",
                    "JavaScript",
                    "React",
                    "Node.js",
                    "MongoDB"
                ],
                "coursePrerequisites": [
                    "Basic computer skills"
                ],
                "educationalLevel": "Beginner",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.9",
                    "reviewCount": "1250"
                },
                "offers": {
                    "@type": "Offer",
                    "price": "499.99",
                    "priceCurrency": "USD",
                    "availability": "https://schema.org/InStock"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        course = objects[0]
        assert course["@type"] == "Course"
        assert course["name"] == "Complete Web Development Bootcamp"
        assert course["description"] == "From zero to full-stack developer in 12 weeks"
        assert course["url"] == "https://example.com/courses/web-dev-bootcamp"
        assert course["image"] == "https://example.com/images/web-dev-course.jpg"
        assert "provider" in course
        assert "hasCourseInstance" in course
        assert "teaches" in course
        assert "coursePrerequisites" in course
        assert course["educationalLevel"] == "Beginner"
        assert "aggregateRating" in course
        assert "offers" in course


class TestCourseEducationalLevel:
    """Test Course with educational level"""

    def test_course_with_educational_level(self):
        """Test course with educationalLevel field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Advanced Data Structures",
                "educationalLevel": "Advanced",
                "description": "Master complex data structures"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert objects[0]["educationalLevel"] == "Advanced"


class TestCourseWithImages:
    """Test Course with images"""

    def test_course_with_single_image(self):
        """Test course with single image"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Photography Basics",
                "image": "https://example.com/photography-course.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert "image" in objects[0]

    def test_course_with_multiple_images(self):
        """Test course with array of images"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Design Thinking",
                "image": [
                    "https://example.com/img1.jpg",
                    "https://example.com/img2.jpg",
                    "https://example.com/img3.jpg"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Course"
        assert "image" in objects[0]


class TestOnlineCourse:
    """Test realistic online course examples"""

    def test_online_course(self):
        """Test realistic online course scenario"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Machine Learning A-Z",
                "description": "Hands-on Python & R In Data Science",
                "url": "https://example.com/ml-course",
                "image": "https://example.com/ml-course-image.jpg",
                "provider": {
                    "@type": "Organization",
                    "name": "Online Learning Platform",
                    "sameAs": "https://example.com"
                },
                "hasCourseInstance": [
                    {
                        "@type": "CourseInstance",
                        "courseMode": "online",
                        "courseWorkload": "PT44H",
                        "instructor": {
                            "@type": "Person",
                            "name": "Kirill Eremenko"
                        }
                    }
                ],
                "teaches": [
                    "Machine Learning",
                    "Python",
                    "R Programming",
                    "Data Science"
                ],
                "coursePrerequisites": [
                    "High School Math"
                ],
                "educationalLevel": "Intermediate",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.5",
                    "ratingCount": "150889",
                    "bestRating": "5"
                },
                "offers": {
                    "@type": "Offer",
                    "category": "Paid",
                    "price": "84.99",
                    "priceCurrency": "USD"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        course = objects[0]
        assert course["@type"] == "Course"
        assert course["name"] == "Machine Learning A-Z"
        assert "provider" in course
        assert "hasCourseInstance" in course
        assert "teaches" in course
        assert "coursePrerequisites" in course
        assert course["educationalLevel"] == "Intermediate"
        assert "aggregateRating" in course
        assert "offers" in course


class TestCourseIntegration:
    """Test Course integration with extract_all()"""

    def test_extract_all_includes_course(self):
        """Test that extract_all() includes Course"""
        html = """
        <html>
        <head>
            <title>Course Page</title>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Course",
                "name": "Introduction to Programming",
                "description": "Learn programming from scratch",
                "provider": {
                    "@type": "Organization",
                    "name": "Tech School"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "Course"
        assert data["jsonld"][0]["name"] == "Introduction to Programming"
        assert data["jsonld"][0]["description"] == "Learn programming from scratch"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
