import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequest(findTestObject('GET/Get List'))

WS.verifyResponseStatusCode(response, 200)

def slurper = new JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())


for(int i=0; i < result.size(); i++) {
	WS.verifyElementPropertyValue(response, 'data.id', result.data.id)
	WS.verifyElementPropertyValue(response, 'data.name', result.data.name)
	WS.verifyElementPropertyValue(response, 'data.year', result.data.year)
	WS.verifyElementPropertyValue(response, 'data.color', result.data.color)
	WS.verifyElementPropertyValue(response, 'data.pantone_value', result.data.pantone_value)
}
WS.verifyElementPropertyValue(response, 'page', 1)
WS.verifyElementPropertyValue(response, 'per_page', 6)
WS.verifyElementPropertyValue(response, 'total', 12)
WS.verifyElementPropertyValue(response, 'total_pages', 2)
WS.verifyElementPropertyValue(response, 'support.url', "https://reqres.in/#support-heading")
WS.verifyElementPropertyValue(response, 'support.text', "To keep ReqRes free, contributions towards server costs are appreciated!")