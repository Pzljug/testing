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

WebDriver driver = Driver.Factory.getWebDriver()

WebElement Table = driver.findElement(By.xpath('//table/tbody'))

List<WebElement> rows_table = Table.findElements(By.tagName('tr'))

int rows_count = rows_table.size()

'Loop'
Loop: for (int row = 0; row < row_count; row++) {
    List<WebElement> Columns_row = rows_table.get(row).findElements(By.tagName('td'))

    int columns_count = Columns_row.size()

    println((('Number of cells In Row ' + row) + ' are ') + columns_count)

    for (int column = 0; column < columns_count; column++) {
        String celltext = Columns_row.get(column).getText()

        println((((('Cell Value Of row number ' + row) + ' and column number ') + column) + ' Is ') + celltext)
    }
}

