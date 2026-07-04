(ns crud-be-cjpd.config-test
  (:require [clojure.test :refer [deftest testing is]]
            [clojure.java.io :as io]
            [clojure.string :as str]
            [crud-be-cjpd.config :as sut]))

(def ^:private base-env
  {"CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET" "test-jwt-secret-at-least-32-chars!!"})

(deftest load-config-test
  (testing "config source reads CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET env var name"
    (let [src (slurp (io/resource "crud_be_cjpd/config.clj"))]
      (is (str/includes? src "CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET"))
      (is (not (str/includes? src "APP_JWT_SECRET")))))

  (testing "config source reads CRUD_BE_CLOJURE_PEDESTAL_PORT env var name"
    (let [src (slurp (io/resource "crud_be_cjpd/config.clj"))]
      (is (str/includes? src "CRUD_BE_CLOJURE_PEDESTAL_PORT"))))

  (testing "config source has no fallback default for jwt-secret"
    (let [src (slurp (io/resource "crud_be_cjpd/config.clj"))]
      (is (not (str/includes? src "default-dev-secret-change-in-production"))
          "jwt-secret must have no soft default — absent var must cause a hard error")))

  (testing "load-config reads jwt-secret from CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET"
    (let [cfg (sut/load-config base-env)]
      (is (= "test-jwt-secret-at-least-32-chars!!" (:jwt-secret cfg)))
      (is (= 8201 (:port cfg)))))

  (testing "load-config throws when CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET is absent"
    (is (thrown? clojure.lang.ExceptionInfo
                 (sut/load-config {"CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET" nil})))))
