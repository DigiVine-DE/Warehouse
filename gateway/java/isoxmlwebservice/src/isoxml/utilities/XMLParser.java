package isoxml.utilities;


import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;

import org.apache.log4j.Logger;
import org.jdom.Document;
import org.jdom.Element;
import org.jdom.JDOMException;
import org.jdom.input.SAXBuilder;
import org.jdom.xpath.XPath;

import isoxml.exception.XMLException;


/**
 * XMLParser
 *
 */
public class XMLParser {
	
	@SuppressWarnings ("unused")
  private static Logger utilitiesLogger = Logger.getLogger(XMLParser.class.getName());
	
  public static HashMap <String, String> getKonfigurationMap(String dateiPfad, String bereich) throws XMLException {
		HashMap <String, String> konfigurationMap = new HashMap <String, String> ();
		try {
			
			Document doc = openDokument(dateiPfad);
						
			Element parent = (Element) XPath.selectSingleNode(doc, bereich); 
			parent.getText();
			
			@SuppressWarnings ("unchecked")
			List <Element> parameterList = (List<Element>) parent.getChildren();
			
			Iterator <Element> iter = parameterList.iterator();
			
			while (iter.hasNext()) {
				Element parameter = (Element)iter.next();
				konfigurationMap.put(parameter.getAttribute("name").getValue(), parameter.getTextNormalize());
			}
		} catch (XMLException e) {
			throw new XMLException ("XMLException: "+e.getMessage());
		} catch (JDOMException e) {
			throw new XMLException ("JDOMException: " + e.getMessage());
		}
		return konfigurationMap;
	}
	
	public static Document openDokument (String url) throws XMLException {
		Document doc = null;
		SAXBuilder builder = new SAXBuilder();
		try {
			doc = builder.build (url);
		} catch (IOException io) {
			io.printStackTrace();
			//throw new XMLException(io.getMessage());
		}catch (JDOMException jdome) {
			//jdome.printStackTrace();
			throw new XMLException(jdome.getMessage());
		}
		return doc;
	}
	
	public static Document openDokument (InputStream is) throws XMLException {
    Document doc = null;
    SAXBuilder builder = new SAXBuilder();
    try {
      doc = builder.build (is);
    } catch (IOException io) {
      io.printStackTrace();
      //throw new XMLException(io.getMessage());
    }catch (JDOMException jdome) {
      //jdome.printStackTrace();
      throw new XMLException(jdome.getMessage());
    }
    return doc;
  }
	
	public static String getPackageClassName(){
		return XMLParser.class.getName();
	}
}
