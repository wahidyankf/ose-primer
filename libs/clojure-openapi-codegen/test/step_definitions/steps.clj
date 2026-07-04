(ns step-definitions.steps
  "Cucumber step definitions for
   specs/libs/clojure-openapi-codegen/behavior/gherkin (symlinked at
   test/features). Every step calls the real openapi-codegen.core/generate
   and openapi-codegen.generator/openapi-type->malli functions against the
   real bundled OpenAPI spec — no mocks, no stubs."
  (:require [clojure.edn :as edn]
            [clojure.string :as str]
            [clojure.test :refer [is]]
            [openapi-codegen.core :as core]
            [openapi-codegen.generator :as generator]
            [openapi-codegen.parser :as parser]
            [lambdaisland.cucumber.dsl :refer [Given When Then]]))

(def ^:private bundled-spec-path
  "../../specs/apps/crud/containers/contracts/generated/openapi-bundled.yaml")

(defn- make-temp-dir
  "Create a temporary directory and return its path string."
  []
  (let [d (java.io.File. (System/getProperty "java.io.tmpdir")
                         (str "codegen-bdd-" (System/nanoTime)))]
    (.mkdirs d)
    (.deleteOnExit d)
    (.getAbsolutePath d)))

(defn- extract-schema-form
  "Read every top-level form in the generated schema file at file-path and
  return the value bound by its `(def schema ...)` form."
  [file-path]
  (let [source (slurp file-path)
        rdr    (java.io.PushbackReader. (java.io.StringReader. source))]
    (loop []
      (let [form (edn/read {:eof ::eof} rdr)]
        (cond
          (= form ::eof) nil
          (and (list? form) (= 'def (first form))) (nth form 2)
          :else (recur))))))

;; ============================================================
;; Scenario: Generating a schema with required and optional properties
;; ============================================================

(Given "a bundled OpenAPI spec whose {string} schema requires {string} and {string}"
       [state schema-name field-a field-b]
       (let [schemas (parser/parse-schemas bundled-spec-path)
             schema  (first (filter #(= schema-name (:name %)) schemas))]
         (is (some? schema) (str "Expected schema " schema-name " in bundled spec"))
         (is (contains? (:required schema) field-a)
             (str schema-name " should require " field-a))
         (is (contains? (:required schema) field-b)
             (str schema-name " should require " field-b))
         (assoc state
                :spec-path bundled-spec-path
                :schema-name schema-name
                :output-dir (make-temp-dir))))

(When "I call generate with the spec path and an output directory" [state]
      (let [paths (core/generate (:spec-path state) (:output-dir state))]
        (assoc state :generated-paths paths)))

(Then "a file is written at {string}" [state expected-path]
      (let [relative  (str/replace-first expected-path #"^output-dir/" "")
            full-path (str (:output-dir state) "/" relative)
            file      (java.io.File. full-path)]
        (is (.exists file) (str "Expected generated file at " full-path))
        (is (some #(= full-path %) (:generated-paths state))
            (str "Expected " full-path " to be among generated paths "
                 (:generated-paths state)))
        (assoc state :checked-file full-path)))

(Then "the file's schema def is a Malli {string} form" [state expected-form-str]
      (let [actual-form   (extract-schema-form (:checked-file state))
            expected-form (edn/read-string expected-form-str)]
        (is (= expected-form actual-form)
            (str "Expected schema form " expected-form " but got " actual-form))
        state))

;; ============================================================
;; Scenario Outline: OpenAPI types map to their corresponding Malli type
;; ============================================================

(Given "an OpenAPI property of type {string}" [state openapi-type]
       (assoc state :openapi-type openapi-type))

(When "I call openapi-type->malli on the property" [state]
      (assoc state :malli-result (generator/openapi-type->malli (:openapi-type state))))

(Then "the result is the Malli type {string}" [state expected-malli-type]
      (let [expected (edn/read-string expected-malli-type)]
        (is (= expected (:malli-result state))
            (str "Expected " expected " but got " (:malli-result state)))
        state))
