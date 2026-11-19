FROM nginx:1.27-alpine

RUN apk add --no-cache nginx python3 openssl

# Generate a self-signed ECDSA P-256 (prime256v1) certificate
RUN mkdir -p /etc/nginx/ssl && \
    openssl req -x509 -nodes -days 365 \
    -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 \
    -keyout /etc/nginx/ssl/server.key \
    -out /etc/nginx/ssl/server.crt \
    -subj "/CN=10.242.149.60" \
    -addext "subjectAltName=IP:10.242.149.60,DNS:localhost"

COPY nginx.conf /etc/nginx/nginx.conf
COPY server.py /app/server.py
COPY start.sh /app/start.sh

RUN chmod +x /app/start.sh

EXPOSE 443

CMD ["/app/start.sh"]
