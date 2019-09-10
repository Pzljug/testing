import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static org.junit.Assert.*
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.waitForPageLoad(5)

WebUI.setText(findTestObject('Object Repository/Page_Masquerade  MHCDEV/input__select2-search__field'), 'Ellen Mastrangelo')

WebUI.click(findTestObject('Object Repository/Page_Masquerade  MHCDEV/span_Ellen Mastrangelo (ellenmastrangelouhhospitalsorg)'))

WebUI.click(findTestObject('Object Repository/Page_Health System Home  MHCDEV/span_Entities'))

WebUI.click(findTestObject('Object Repository/Page_Entities  MHCDEV/a_University Hospitals Ahuja Medical Center'))

WebUI.click(findTestObject('Object Repository/users_Dash/Reporting_button'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Reporting Dashboard/a_Requirement Report'))

WebUI.click(findTestObject('Object Repository/Page_Requirement Report - by rotation/a_View by Rotation'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/ul__select2-selection__rendered'))

WebUI.setText(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/input__select2-search__field'), 
    'Clinical Faculty')

WebUI.sendKeys(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/input__select2-search__field'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/span_Please select a requirement'))

WebUI.setText(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/input__select2-search__field'), 
    'TB - UH Employees Only')

WebUI.sendKeys(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/input__select2-search__field'), 
    Keys.chord(Keys.ENTER))

'Incomplete'
WebUI.click(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/ul__select2-selection__rendered_1'))

WebUI.click(findTestObject('Object Repository/Page_Requirement Report - by rotation  MHCDEV/button_Run Report'))

WebUI.closeBrowser()

