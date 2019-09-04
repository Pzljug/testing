import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW

WebUI.openBrowser('')

WebUI.navigateToUrl('https://mhcdev.com/login')

WebUI.setEncryptedText(findTestObject('Page_Login  MHCDEV/input_Password_login_password'), 'h8Chv16IRUTpZZOGPKpI6cNl0s63PdN7')

WebUI.setText(findTestObject('Object Repository/Page_Login  MHCDEV/input_Email Address_login_email'), 'brian.clauser@mhc.org')

WebUI.click(findTestObject('Object Repository/Page_Login  MHCDEV/button_Login'))

WebUI.click(findTestObject('Object Repository/Page_Admin Dashboard  MHCDEV/a_Members'))

WebUI.click(findTestObject('Object Repository/Page_Admin Members  MHCDEV/a_Add Student'))

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_first_name'), 'QA')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_middle_initial'), 'N/A')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_last_name'), 'Test##')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_address_street_1'), '123 Someplace')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_address_city'), 'Nowhere')

WebUI.click(findTestObject('Object Repository/Page_Create New Member  MHCDEV/span_-Select a State-'))

WebUI.setText(findTestObject('Page_Create New Member  MHCDEV/input__select2-search__field'), 'Michigan')

WebUI.sendKeys(findTestObject('Page_Create New Member  MHCDEV/input__select2-search__field'), '${KEY_ENTER}')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_address_zip'), '48855')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_phone'), '1234567890')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__member_dob'), '12/30/1974')

WebUI.setText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input__email_address'), 'qa.test##@example.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input_Password is not set_password_1'), 
    '6KNwbqGrpUO7MUb8YgKQMTfCxw8XE3cJ')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input_Score4_password_2'), '6KNwbqGrpUO7MUb8YgKQMTfCxw8XE3cJ')

WebUI.click(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input_Example_FC_SAVE_BTN'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Create New Member  MHCDEV/input_Password is not set_password_1'), 
    '6KNwbqGrpUO7MUb8YgKQMTfCxw8XE3cJ')

