import com.kms.katalon.core.annotation.Keyword as Keyword
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
import org.junit.After as After
import org.openqa.selenium.By as By
import org.openqa.selenium.WebDriver as WebDriver
import org.testng.Assert as Assert
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
import org.openqa.selenium.WebElement as WebElement

'Define Custom Path where file needs to be download'
String downloadPath = 'C:\\Users\\bclauser\\Documents\\FileDownloadChecking'

WebUI.callTestCase(findTestCase('Masq'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Page_Masquerade  MHCDEV/input__select2-search__field'), 'Marieta Marcial')

WebUI.click(findTestObject('Page_Masquerade  MHCDEV/span_Marieta Marcial (marietamarcialphoenixedu)'))

WebUI.click(findTestObject('Page_Entities  MHCDEV/a_University of Phoenix FNP - Online'))

'Clicks the More dropdown button.\t'
WebUI.click(findTestObject('Page_University of Phoenix FNP - Online  MHCDEV/a_More'))

'Clicks Time Log button'
WebUI.click(findTestObject('Page_University of Phoenix FNP - Online  MHCDEV/a_Time Logs'))

WebUI.click(findTestObject('Page_Time Logs  MHCDEV/a_List'))

WebUI.delay(10)

WebUI.click(findTestObject('Page_FNP (Assigned)  MHCDEV/button_Generate CSV'))

WebUI.delay(10)

WebUI.click(findTestObject('Page_FNP (Assigned)  MHCDEV/button_Download Generated CSV'))

WebUI.delay(10)

Assert.assertTrue(isFileDownloaded('C:\\Users\\bclauser\\Documents\\FileDownloadChecking', '.csv'))

boolean isFileDownloaded(String downloadPath, String fileName) {
    boolean flag = false

    'Creating an object for File and passing the download Path as argument'
    File dir = new File(downloadPath)

    'Creating an Array where it will store all the files from that folder'
    File[] dir_contents = dir.listFiles()

    for (int i = 0; i < dir_contents.length; i++) {
        println('File Name at 0 is : ' + dir_contents[i].getName())

        'Verifying the file name is available in the folder '
        if (dir_contents[i].getName().contains(fileName)) {
            'If the file is found then it will return a value as true'
            return flag = true
        }
    }  
    'If the file is not found then it will return a value as false'
    return flag
    
}
WebUI.closeBrowser()
