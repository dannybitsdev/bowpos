# Bowpos Docker Setup

Este proyecto está preparado para ejecutarse con Docker Compose usando un stack de desarrollo basado en:

- Backend: Rust + Axum + SQLx
- Frontend: React + Vite + PNPM
- Base de datos: PostgreSQL

## Estructura esperada

```text
bowpos/
├── docker-compose.yml
├── .env
├── .env.example
├── .dockerignore
├── backend/
│   ├── Dockerfile
│   └── migrations/
│       └── 0001_initial_schema.sql
└── frontend/
    └── Dockerfile
```

## Requisitos

- Docker
- Docker Compose v2

## Variables de entorno

Copia el archivo de ejemplo y ajústalo si es necesario:

```bash
cp .env.example .env
```

Variables principales:

```env
POSTGRES_USER=appuser
POSTGRES_PASSWORD=changeme
POSTGRES_DB=appdb
POSTGRES_PORT=5432

DATABASE_URL=postgresql://appuser:changeme@db:5432/appdb?schema=public
BACKEND_PORT=8080
FRONTEND_PORT=3000
RUST_LOG=info
```

## Levantar el entorno de desarrollo

```bash
docker compose up --build
```

## Ejecutar migraciones

Las migraciones se aplican automáticamente al iniciar el backend, leyendo los archivos SQL de la carpeta `backend/migrations/`.

Si quieres asegurarte de que se ejecuten al levantar los servicios, usa:

```bash
docker compose up --build backend
```

Si ya estaba levantado y deseas reiniciarlo para volver a aplicar las migraciones:

```bash
docker compose restart backend
```

Servicios disponibles:

- Frontend: http://localhost:3000
- Backend: http://localhost:8080
- PostgreSQL: localhost:5432

## Servicios incluidos

### Base de datos
- Imagen: `postgres:16-alpine`
- Datos persistidos en un volumen Docker llamado `postgres_data`

### Backend
- Usa `cargo-watch` para recargar automáticamente al detectar cambios en Rust
- Espera a PostgreSQL mediante healthcheck antes de iniciar

### Frontend
- Usa Vite para desarrollo con recarga en caliente
- Instala dependencias con PNPM y las monta de forma segura en un volumen interno

## Detener los servicios

```bash
docker compose down
```

## Limpiar volúmenes y datos

```bash
docker compose down -v
```

## Build para producción

El Dockerfile del backend y del frontend también soportan builds de producción.

```bash
docker compose build backend frontend
```

## Despliegue en VPS con Nginx y HTTPS

Si vas a desplegar este stack en un servidor remoto, puedes usar Nginx como proxy inverso.

### 1. Instalar Nginx

```bash
sudo apt update && sudo apt install nginx certbot python3-certbot-nginx -y
```

### 2. Copiar la configuración

```bash
sudo cp nginx.conf /etc/nginx/conf.d/bowpos.conf
sudo nginx -t
sudo systemctl reload nginx
```

### 3. Configurar HTTPS

```bash
sudo certbot --nginx -d your-domain.com -d www.your-domain.com -d api.your-domain.com
```

### 4. Levantar los contenedores en producción

```bash
cp .env.production.example .env.production
sudo docker compose -f docker-compose.prod.yml up -d --build
```

### 5. Verificar

```bash
sudo docker compose -f docker-compose.prod.yml ps
```

## Despliegue alternativo con Traefik y HTTPS automático

Para un despliegue más moderno y con emisión automática de certificados TLS, puedes usar Traefik.

### 1. Preparar el archivo de entorno

```bash
cp .env.traefik.example .env.traefik
```

### 2. Ajustar los valores

Edita `.env.traefik` y reemplaza:

- `ACME_EMAIL` por tu correo
- `DOMAIN` por tu dominio principal

### 3. Levantar Traefik y los servicios

```bash
sudo docker compose --env-file .env.traefik -f docker-compose.traefik.yml up -d --build
```

### 4. Verificar

```bash
sudo docker compose --env-file .env.traefik -f docker-compose.traefik.yml ps
```

## Notas importantes

- El frontend usa PNPM, no NPM.
- Los cambios en el código se reflejan automáticamente en modo desarrollo.
- El backend espera a que PostgreSQL esté listo antes de iniciar, lo que ayuda con SQLx y migraciones.
