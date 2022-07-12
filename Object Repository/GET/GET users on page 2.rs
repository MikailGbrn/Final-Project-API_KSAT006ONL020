<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET users on page 2</name>
   <tag></tag>
   <elementGuidId>13d4510f-842c-477d-a8fc-2a51dc012685</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def slurper = new JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

WS.verifyElementPropertyValue(response, 'page', 2)
WS.verifyElementPropertyValue(response, 'per_page', 6)
WS.verifyElementPropertyValue(response, 'total', 12)
WS.verifyElementPropertyValue(response, 'total_pages', 2)

for(int i=0; i &lt; result.size(); i++) {
	WS.verifyElementPropertyValue(response, 'data[' + i + '].id', result.data[i].id)
	WS.verifyElementPropertyValue(response, 'data[' + i + '].email', result.data[i].email)
	WS.verifyElementPropertyValue(response, 'data[' + i + '].first_name', result.data[i].first_name)
	WS.verifyElementPropertyValue(response, 'data[' + i + '].last_name', result.data[i].last_name)
	WS.verifyElementPropertyValue(response, 'data[' + i + '].avatar', result.data[i].avatar)
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
