(ns crud-be-cjpd.config
  "Load and validate configuration from environment variables."
  (:require [malli.core :as m]
            [crud-be-cjpd.domain.schemas :as schemas]))

(defn load-config
  "Return application configuration from environment variables.
   Optional env-overrides map substitutes specific keys (for testing).
   Validates the result against schemas/Config."
  ([] (load-config {}))
  ([env-overrides]
   (let [getenv (fn [k] (or (get env-overrides k) (System/getenv k)))
         jwt-secret (or (getenv "CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET")
                        (throw (ex-info "CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET is required" {})))
         config {:port         (Integer/parseInt (or (getenv "CRUD_BE_CLOJURE_PEDESTAL_PORT") "8201"))
                 :database-url (or (getenv "DATABASE_URL") "jdbc:sqlite::memory:")
                 :jwt-secret   jwt-secret}]
     (assert (m/validate schemas/Config config)
             (str "Invalid configuration: " (pr-str (m/explain schemas/Config config))))
     config)))
