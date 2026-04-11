/**
 * MetaOxide WASM - Vercel Edge Functions Example
 *
 * Serverless edge computing example for extracting metadata at the edge.
 * Deploy this function to Vercel for global low-latency metadata extraction.
 *
 * Deployment:
 *   1. Create api/extract.ts in your Vercel project with this content
 *   2. Add to package.json: "@yfedoseev/meta-oxide-wasm": "^0.1.3"
 *   3. Deploy: vercel --prod
 *
 * File structure:
 *   api/
 *     extract.ts (this file)
 *   vercel.json (optional config)
 *
 * Usage:
 *   GET  /api/extract?url=https://example.com
 *   POST /api/extract (with JSON body: { html, baseUrl })
 *
 * Features:
 *   - Extract metadata from URLs
 *   - Direct HTML extraction
 *   - JSON API with metadata enrichment
 *   - Edge runtime for low latency
 *   - Rate limiting friendly
 */

import { extractAll, initialize } from '@yfedoseev/meta-oxide-wasm';
import type { ExtractionResult } from '@yfedoseev/meta-oxide-wasm';

// Edge runtime configuration
export const config = {
    runtime: 'edge',
};

interface ApiResponse {
    success: boolean;
    data?: ExtractionResult;
    meta?: {
        url?: string;
        htmlSize?: number;
        duration?: string;
        timestamp?: string;
        region?: string;
        baseUrl?: string | null;
    };
    error?: string;
}

/**
 * Create JSON response with CORS headers
 */
function jsonResponse(data: ApiResponse, status: number = 200): Response {
    return new Response(JSON.stringify(data, null, 2), {
        status,
        headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
            'Access-Control-Allow-Headers': 'Content-Type',
            'Cache-Control': 'public, max-age=3600', // Cache for 1 hour
        },
    });
}

/**
 * Extract metadata from URL
 */
async function extractFromUrl(url: string): Promise<ApiResponse> {
    const startTime = Date.now();

    try {
        // Validate URL
        new URL(url);
    } catch {
        return {
            success: false,
            error: 'Invalid URL format',
        };
    }

    try {
        // Fetch HTML
        const response = await fetch(url, {
            headers: {
                'User-Agent': 'MetaOxide/1.0 (Vercel Edge Function)',
            },
            // Timeout after 10 seconds
            signal: AbortSignal.timeout(10000),
        });

        if (!response.ok) {
            return {
                success: false,
                error: `Failed to fetch: ${response.status} ${response.statusText}`,
            };
        }

        const contentType = response.headers.get('content-type') || '';
        if (!contentType.includes('text/html')) {
            return {
                success: false,
                error: `Expected HTML content, got ${contentType}`,
            };
        }

        const html = await response.text();

        // Extract metadata
        const data = await extractAll(html, { baseUrl: url });

        const duration = Date.now() - startTime;

        return {
            success: true,
            data,
            meta: {
                url,
                htmlSize: html.length,
                duration: `${duration}ms`,
                timestamp: new Date().toISOString(),
                region: process.env.VERCEL_REGION || 'unknown',
            },
        };

    } catch (error) {
        const message = error instanceof Error ? error.message : 'Unknown error';
        return {
            success: false,
            error: `Extraction failed: ${message}`,
        };
    }
}

/**
 * Extract metadata from HTML body
 */
async function extractFromHtml(html: string, baseUrl?: string): Promise<ApiResponse> {
    const startTime = Date.now();

    if (!html || html.length === 0) {
        return {
            success: false,
            error: 'Empty HTML content',
        };
    }

    if (html.length > 5_000_000) {
        return {
            success: false,
            error: 'HTML content too large (max 5MB)',
        };
    }

    try {
        const data = await extractAll(html, { baseUrl });

        const duration = Date.now() - startTime;

        return {
            success: true,
            data,
            meta: {
                htmlSize: html.length,
                duration: `${duration}ms`,
                timestamp: new Date().toISOString(),
                region: process.env.VERCEL_REGION || 'unknown',
                baseUrl: baseUrl || null,
            },
        };

    } catch (error) {
        const message = error instanceof Error ? error.message : 'Unknown error';
        return {
            success: false,
            error: `Extraction failed: ${message}`,
        };
    }
}

/**
 * Main Edge Function handler
 */
export default async function handler(request: Request): Promise<Response> {
    // Initialize WASM (cached after first call)
    await initialize();

    const { method } = request;
    const url = new URL(request.url);

    // Handle CORS preflight
    if (method === 'OPTIONS') {
        return new Response(null, {
            status: 204,
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
                'Access-Control-Allow-Headers': 'Content-Type',
            },
        });
    }

    // GET: Extract from URL parameter
    if (method === 'GET') {
        const targetUrl = url.searchParams.get('url');

        if (!targetUrl) {
            return jsonResponse(
                {
                    success: false,
                    error: 'Missing "url" query parameter',
                },
                400
            );
        }

        const result = await extractFromUrl(targetUrl);
        return jsonResponse(result, result.success ? 200 : 400);
    }

    // POST: Extract from HTML body
    if (method === 'POST') {
        try {
            const contentType = request.headers.get('content-type') || '';

            if (contentType.includes('application/json')) {
                const body = (await request.json()) as {
                    html: string;
                    baseUrl?: string;
                };

                if (!body.html) {
                    return jsonResponse(
                        {
                            success: false,
                            error: 'Missing "html" field in JSON body',
                        },
                        400
                    );
                }

                const result = await extractFromHtml(body.html, body.baseUrl);
                return jsonResponse(result, result.success ? 200 : 400);

            } else if (contentType.includes('text/')) {
                const html = await request.text();
                const baseUrl = url.searchParams.get('baseUrl') || undefined;

                const result = await extractFromHtml(html, baseUrl);
                return jsonResponse(result, result.success ? 200 : 400);

            } else {
                return jsonResponse(
                    {
                        success: false,
                        error: 'Unsupported content type. Use application/json or text/html',
                    },
                    400
                );
            }

        } catch (error) {
            return jsonResponse(
                {
                    success: false,
                    error: 'Invalid request body',
                },
                400
            );
        }
    }

    return jsonResponse(
        {
            success: false,
            error: 'Method not allowed. Use GET or POST',
        },
        405
    );
}
