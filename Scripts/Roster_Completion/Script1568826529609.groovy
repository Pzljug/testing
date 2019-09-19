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

WebUI.click(findTestObject('Object Repository/Page_Admin Dashboard  MHCDEV/a_Misc Functions'))

WebUI.click(findTestObject('Object Repository/Page_Admin Misc Functions  MHCDEV/span_Masquerade'))

WebUI.click(findTestObject('Page_Masquerade  MHCDEV/button_Ajay Arumugam'))

WebUI.click(findTestObject('Page_Admin Dashboard  MHCDEV/span_Ajay ArumugamAccount Menu'))

WebUI.click(findTestObject('Page_Admin Dashboard  MHCDEV/a_School User'))

WebUI.click(findTestObject('Page_School Home  MHCDEV/a_ACE College'))

WebUI.click(findTestObject('Page_ACE College  MHCDEV/a_Create Rotation'))

WebUI.click(findTestObject('Page_Create New Rotation  MHCDEV/span_Select an Entity'))

WebUI.setText(findTestObject('Page_Create New Rotation  MHCDEV/input__select2-search__field'), 'Ace hospital')

WebUI.delay(5)

WebUI.sendKeys(findTestObject('Page_Create New Rotation  MHCDEV/input__select2-search__field'), Keys.chord(Keys.ENTER))

WebUI.verifyElementAttributeValue(findTestObject('Page_Create New Rotation  MHCDEV/input__select2-search__field'), 'Ace hospital', 
    '1818', 0)

WebUI.click(findTestObject('Page_Create New Rotation  MHCDEV/button_Save  Continue'))

WebUI.click(findTestObject('Page_Create New Rotation  MHCDEV/span_Program'))

WebUI.setText(findTestObject('Object Repository/Page_Create New Rotation  MHCDEV/input_entityschool_select2-search__field'), 
    'bsn')

WebUI.sendKeys(findTestObject('Object Repository/Page_Create New Rotation  MHCDEV/input_entityschool_select2-search__field'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Page_Create New Rotation  MHCDEV/span_Experience'))

WebUI.setText(findTestObject('Object Repository/Page_Create New Rotation  MHCDEV/input_entityschool_select2-search__field'), 
    'coh')

WebUI.sendKeys(findTestObject('Object Repository/Page_Create New Rotation  MHCDEV/input_entityschool_select2-search__field'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Page_Create New Rotation  MHCDEV/button_Save'))

WebUI.click(findTestObject('Page_Create New Rotation  MHCDEV/button_Confirm'))

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/span_Unit'))

WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input_entityschool210rotationsaddqAcereqlevel22122oldTabindex0_1568833743333_select2-search__field'), 
    '2 South')

WebUI.sendKeys(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input_entityschool210rotationsaddqAcereqlevel22122oldTabindex0_1568833743333_select2-search__field'), 
    Keys.chord(Keys.ENTER))

if (browserName == "FIREFOX_DRIVER"){
	WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_start'), '2019-09-20')
}else{
WebUI.sendKeys(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_start'), Keys.chord('09-20-2019', Keys.ENTER, Keys.TAB))


//WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/div_Start Date'))

//WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_start'))

//WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/th_Today'))

if (browserName == "FIREFOX_DRIVER"){
	WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_end'), '2019-09-20')
}else{
WebUI.sendKeys(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_end'), Keys.chord('09-20-2019', Keys.ENTER, Keys.TAB))

//WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_start'), '09/18/2019')

//WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/div_End Date'))

//WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_end'))

//WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/th_Today'))

//WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_date_start'), '09/18/2019')

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/div_Days of the Week'))

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/ul__select2-selection__rendered'))

WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__select2-search__field'), 'Monday')

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/span_Not Set'))

WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input_entityschool210rotationsaddqAcereqlevel22122oldTabindex0_1568833743333_select2-search__field'), 
    'Community')

WebUI.sendKeys(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input_entityschool210rotationsaddqAcereqlevel22122oldTabindex0_1568833743333_select2-search__field'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/span_Status'))

WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input_concat(id(  select2-id_status-container  ))_select2-search__field'), 
    'Approved')

WebUI.sendKeys(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input_concat(id(  select2-id_status-container  ))_select2-search__field'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input_No_FC_SAVE_BTN'))

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/div_An error was encountered'))

WebUI.click(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/i_entityschool210rotationsaddqAcereqlevel22122oldTabindex0_1568833743333_fa fa-fw fa-times'))

WebUI.setText(findTestObject('Object Repository/Page_Manage Rotation 175291  MHCDEV/input__rotation_student_slots'), '2')

