import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Log In'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.waitForPageLoad(5)

WebUI.click(findTestObject('Object Repository/Page_Admin Dashboard  MHCDEV/a_EXTREMELY DANGEROUS'))

WebUI.waitForElementClickable(findTestObject('Page_Admin EXTREMELY DANGEROUS  MHCDEV/i_EXTREMELY DANGEROUS_danger-icon'), 
    5)

WebUI.click(findTestObject('Object Repository/Page_Admin EXTREMELY DANGEROUS  MHCDEV/i_EXTREMELY DANGEROUS_danger-icon'))

WebUI.waitForElementVisible(findTestObject('Page_Admin EXTREMELY DANGEROUS  ACEMAPP/a_General Member Test'), 30000)

WebUI.click(findTestObject('Page_Admin EXTREMELY DANGEROUS  ACEMAPP/a_General Member Test'))

WebUI.waitForElementClickable(findTestObject('Page_General Member Tester  MHCDEV/span_Brian ClauserAccount Menu'), 5)

WebUI.click(findTestObject('Object Repository/Page_General Member Tester  MHCDEV/span_Brian ClauserAccount Menu'))

WebUI.click(findTestObject('Object Repository/Page_General Member Tester  MHCDEV/a_General Member'))

WebUI.verifyElementNotPresent(findTestObject('billingPanelGeneral'), 50)

WebUI.verifyElementPresent(findTestObject('GeneralMemberNotificationPanel'), 30)

