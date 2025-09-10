# Logging-Service
# Logging Service  Centralized logging API for the **Gitdigital Products** ecosystem.   All services push logs here → single source of truth for debugging.  ## 🚀 Features - `POST /log` → Accept JSON log entries - Timestamps added automatically (UTC RFC3339) - In-memory store (upgradeable to DB / Elasticsearch later)  
