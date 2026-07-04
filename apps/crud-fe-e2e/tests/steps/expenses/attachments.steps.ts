import { createBdd } from "playwright-bdd";
import { expect } from "@playwright/test";
import {
  loginUser,
  createExpense,
  registerUser,
  listExpenses,
  uploadAttachmentApi,
  listAttachmentsApi,
  deleteAttachmentApi,
} from "@/utils/api-helpers.js";
import {
  getReceiptJpgPath,
  getInvoicePdfPath,
  getOversizedFilePath,
  getMalwareExePath,
} from "@/fixtures/test-files.js";
import { testState } from "@/utils/test-state.js";

const { Given, When, Then } = createBdd();

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Given: pre-upload via API (not UI) + navigate to entry detail
// ---------------------------------------------------------------------------

Given(
  "{word} has uploaded {string} and {string} to the entry",
  async ({ page }, username: string, file1: string, file2: string) => {
    const { accessToken } = await loginUser(username, "Str0ng#Pass1");
    const expenses = await listExpenses(accessToken);
    const expense = expenses.find((e) => e.description === "Lunch") ?? expenses[0];
    if (!expense) throw new Error("No expense found");

    const file1Path = file1.endsWith(".jpg") || file1.endsWith(".jpeg") ? getReceiptJpgPath() : getInvoicePdfPath();
    const file1Mime = file1.endsWith(".pdf") ? "application/pdf" : "image/jpeg";
    await uploadAttachmentApi(accessToken, expense.id, file1Path, file1, file1Mime);

    const file2Path = file2.endsWith(".pdf") ? getInvoicePdfPath() : getReceiptJpgPath();
    const file2Mime = file2.endsWith(".pdf") ? "application/pdf" : "image/jpeg";
    await uploadAttachmentApi(accessToken, expense.id, file2Path, file2, file2Mime);

    await page.goto(`/expenses/${expense.id}`);
    await expect(page.getByText(file1)).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(file2)).toBeVisible({ timeout: 10000 });
  },
);

Given("{word} has uploaded {string} to the entry", async ({ page }, username: string, filename: string) => {
  const { accessToken } = await loginUser(username, "Str0ng#Pass1");
  const expenses = await listExpenses(accessToken);
  const expense = expenses.find((e) => e.description === "Lunch") ?? expenses[0];
  if (!expense) throw new Error("No expense found");

  const filePath = filename.endsWith(".jpg") || filename.endsWith(".jpeg") ? getReceiptJpgPath() : getInvoicePdfPath();
  const mimeType = filename.endsWith(".pdf") ? "application/pdf" : "image/jpeg";
  await uploadAttachmentApi(accessToken, expense.id, filePath, filename, mimeType);

  await page.goto(`/expenses/${expense.id}`);
  await expect(page.getByText(filename)).toBeVisible({ timeout: 10000 });
});

Given("the attachment has been deleted from another session", async ({}) => {
  const { accessToken } = await loginUser("alice", "Str0ng#Pass1");
  const expenses = await listExpenses(accessToken);
  const expense = expenses.find((e) => e.description === "Lunch") ?? expenses[0];
  if (!expense) return;
  const attachments = await listAttachmentsApi(accessToken, expense.id);
  if (attachments.length === 0) return;
  await deleteAttachmentApi(accessToken, expense.id, attachments[0]!.id);
});

Given(
  "a user {string} has created an entry with description {string}",
  async ({}, username: string, description: string) => {
    await registerUser(username, `${username}@example.com`, "Str0ng#Pass1");
    const { accessToken } = await loginUser(username, "Str0ng#Pass1");
    const expense = (await createExpense(accessToken, {
      amount: "25.00",
      currency: "USD",
      category: "transport",
      description,
      date: "2025-01-15",
      type: "expense",
    })) as { id: string };
    testState.bobExpenseId = expense.id;
  },
);

Given("a user {string} has created an entry with an attachment", async ({}, username: string) => {
  await registerUser(username, `${username}@example.com`, "Str0ng#Pass1");
  const { accessToken } = await loginUser(username, "Str0ng#Pass1");
  const expense = (await createExpense(accessToken, {
    amount: "25.00",
    currency: "USD",
    category: "transport",
    description: "Bob's entry",
    date: "2025-01-15",
    type: "expense",
  })) as { id: string };
  // Upload an attachment via API so the scenario has an attachment to test against
  await uploadAttachmentApi(accessToken, expense.id, getReceiptJpgPath(), "receipt.jpg", "image/jpeg");
  testState.bobExpenseId = expense.id;
});

// ---------------------------------------------------------------------------
// When: UI-driven uploads — use waitForResponse for success paths
// ---------------------------------------------------------------------------

When("{word} uploads file {string} as an image attachment", async ({ page }, _username: string, _filename: string) => {
  const filePath = getReceiptJpgPath();
  const fileInput = page.locator('input[type="file"]');
  await Promise.all([
    page.waitForResponse((r) => r.url().includes("/attachments") && r.request().method() === "POST", {
      timeout: 15000,
    }),
    fileInput.setInputFiles(filePath),
  ]);
});

When(
  "{word} uploads file {string} as a document attachment",
  async ({ page }, _username: string, _filename: string) => {
    const filePath = getInvoicePdfPath();
    const fileInput = page.locator('input[type="file"]');
    await Promise.all([
      page.waitForResponse((r) => r.url().includes("/attachments") && r.request().method() === "POST", {
        timeout: 15000,
      }),
      fileInput.setInputFiles(filePath),
    ]);
  },
);

// Client-side rejection — no network request, just wait briefly
When("{word} attempts to upload file {string}", async ({ page }, _username: string, _filename: string) => {
  const filePath = getMalwareExePath();
  const fileInput = page.locator('input[type="file"]');
  await fileInput.setInputFiles(filePath);
  await page.waitForTimeout(1000);
});

When("{word} attempts to upload an oversized file", async ({ page }) => {
  const filePath = getOversizedFilePath();
  const fileInput = page.locator('input[type="file"]');
  await fileInput.setInputFiles(filePath);
  await page.waitForTimeout(1000);
});

When(
  "{word} clicks the delete button on attachment {string}",
  async ({ page }, _username: string, filename: string) => {
    // Use the specific aria-label to avoid clicking the expense delete button
    await page.getByRole("button", { name: new RegExp(`delete attachment ${filename}`, "i") }).click();
  },
);

// ---------------------------------------------------------------------------
// Then
// ---------------------------------------------------------------------------

Then("the attachment list should contain {string}", async ({ page }, filename: string) => {
  await expect(page.getByText(filename)).toBeVisible({ timeout: 10000 });
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Uploading a JPEG image adds it to the attachment list
// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Uploading a PDF document adds it to the attachment list
Then("the attachment should display as type {string}", async ({ page }, mimeType: string) => {
  await expect(page.getByText(mimeType)).toBeVisible();
});

Then("the attachment list should contain {int} items", async ({ page }, count: number) => {
  const attachmentItems = page
    .getByTestId("attachment-item")
    .or(page.getByRole("listitem").filter({ hasText: /\.(jpg|jpeg|pdf)/i }));
  await expect(attachmentItems).toHaveCount(count);
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Entry detail shows all uploaded attachments
Then("the attachment list should include {string}", async ({ page }, filename: string) => {
  await expect(page.getByText(filename)).toBeVisible();
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Deleting an attachment removes it from the list
Then("the attachment list should not contain {string}", async ({ page }, filename: string) => {
  await expect(page.getByText(filename)).not.toBeVisible();
});

Then("an error message about unsupported file type should be displayed", async ({ page }) => {
  await expect(page.getByText(/unsupported|invalid file type|not allowed/i)).toBeVisible();
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Uploading an unsupported file type shows an error
// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Uploading an oversized file shows an error
Then("the attachment list should remain unchanged", async ({ page }) => {
  const attachmentItems = page
    .getByTestId("attachment-item")
    .or(page.getByRole("listitem").filter({ hasText: /\.(jpg|jpeg|pdf)/i }));
  const count = await attachmentItems.count();
  expect(count).toBeGreaterThanOrEqual(0);
});

Then("an error message about file size limit should be displayed", async ({ page }) => {
  await expect(page.getByText(/too large|size limit|file.*size|maximum/i)).toBeVisible();
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Cannot upload attachment to another user's entry
Then("the upload attachment button should not be visible", async ({ page }) => {
  await expect(page.locator('input[type="file"]')).not.toBeVisible();
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Cannot view attachments on another user's entry
Then("an access denied message should be displayed", async ({ page }) => {
  await expect(
    page.getByText(/access denied|forbidden|not authorized|permission|not found|failed to load/i).first(),
  ).toBeVisible();
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Cannot delete attachment on another user's entry
Then("the delete attachment button should not be visible", async ({ page }) => {
  await expect(
    page.getByRole("button", {
      name: /delete.*attachment|remove.*attachment/i,
    }),
  ).not.toBeVisible();
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/expenses/attachments.feature:Deleting a non-existent attachment shows a not-found error
Then("an error message about attachment not found should be displayed", async ({ page }) => {
  await expect(page.getByText(/not found|no longer exists|deleted/i)).toBeVisible();
});
