import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.annotation.Keyword as Keyword
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser(GlobalVariable.baseurl + "/login")

// Use setText for Firefox and sendKeys for Chrome
browserName = DriverFactory.getExecutedBrowser().getName()
def driver = DriverFactory.getWebDriver()

//String baseUrl = 'https://www.katalon.com/'

WebUI.maximizeWindow()

//selenium = new WebDriverBackedSelenium(driver, baseUrl)
WebUI.click(findTestObject('Page_Login  MHCDEV/input_Email Address_login_email'))

WebUI.setText(findTestObject('Page_Login  MHCDEV/input_Email Address_login_email'), 'brian.clauser@mhc.org')

WebUI.click(findTestObject('Page_Login  MHCDEV/input_Password_login_password'))

WebUI.setEncryptedText(findTestObject('Page_Login  MHCDEV/input_Password_login_password'), 'S8bD7IDnqROwYtWEn+lO31v7zc7gnvPo')

WebUI.click(findTestObject('Page_Login  MHCDEV/button_Login'))

WebUI.waitForPageLoad(30000)

