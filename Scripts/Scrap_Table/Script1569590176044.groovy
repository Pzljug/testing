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
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement

WebUI.callTestCase(findTestCase('Log In'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.maximizeWindow()

WebUI.click(findTestObject('scrapping/Page_Admin Dashboard  MHCDEV/a_Members'))

WebUI.click(findTestObject('scrapping/Page_Admin Members  MHCDEV/input_Member Search_q'))

WebUI.setText(findTestObject('scrapping/Page_Admin Members  MHCDEV/input_Member Search_q'), 'sherry mcmurtrey')

WebUI.sendKeys(findTestObject('scrapping/Page_Admin Members  MHCDEV/input_Member Search_q'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('scrapping/Page_Member Search  MHCDEV/a_Clinical User'))

WebUI.waitForPageLoad(10)

WebUI.click(findTestObject('scrapping/Page_Sherry McMurtrey  MHCDEV/a_Associations'))

WebUI.acceptAlert()

//Extract data from table
List<String> page1 = new ArrayList()
List<String> page2 = new ArrayList()
List<String> page3 = new ArrayList()
List<String> page4 = new ArrayList()
List<String> page5 = new ArrayList()
List<String> page6 = new ArrayList()

List<ArrayList> lArray = new ArrayList<ArrayList>(); //Creates a List of Lists
lArray.add(page1)
lArray.add(page2)
lArray.add(page3)
lArray.add(page4)
lArray.add(page5)
lArray.add(page6)

myObject = new TestObject("myObject")
// Loop paginate
pagi = 1 // paginate start from 1
index = 0
// list for paginate buttons
List<String> list2 = new ArrayList<>()
WebDriver driver2 = DriverFactory.getWebDriver()
'To locate paginate buttons'
list2 = driver2.findElements(By.xpath("//a[containes(@class, 'paginate_button')]"))
println list2.size()
count = list2.size() - 2 // remove precious and next from the count
WebUI.closeBrowser()

// loop paginate
for (int i = 0; 9 < count; i++) {
	
	WebUI.openBrowser('')
	WebUI.	
}
