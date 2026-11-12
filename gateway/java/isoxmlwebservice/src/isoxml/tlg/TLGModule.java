package isoxml.tlg;

import java.io.File;
import java.io.FileNotFoundException;
import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.Calendar;
import java.util.Date;
import java.util.GregorianCalendar;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;

import org.apache.log4j.Logger;
import org.jdom.Attribute;
import org.jdom.Document;
import org.jdom.Element;
import org.jdom.JDOMException;
import org.jdom.input.SAXBuilder;

/**
 * helper for TLG
 *
 */
public class TLGModule {
 private static Logger tlgModuleLogger = Logger.getLogger(TLGBean.class.getName());  
  
 public static void main (String[] args) {
    try {
      getKonfigurationHashMap("/tmp/TLG00001.xml");
    } catch (FileNotFoundException e) {
        tlgModuleLogger.error("main:" + e.getMessage());
    }
 } 
 
  @SuppressWarnings ("unchecked")
  public static LinkedHashMap <String, String> getKonfigurationHashMap(String fileName) throws FileNotFoundException{
    LinkedHashMap <String, String> konfigurationHashMap = new LinkedHashMap <String, String> ();
    try {
      
      Document doc = openDocument(fileName);
      
      //TIM-Element
      Element tim = (Element) doc.getRootElement();
      setAttributesForElement(konfigurationHashMap, tim, null);
      //PTN-Element
      Element ptn = tim.getChild("PTN");
      setAttributesForElement(konfigurationHashMap, ptn, null);
      //DLV-Elements
      List <Element> dlvList = (List <Element>) tim.getChildren("DLV");
      Element dlvElement = null;
      for(int index = 0; index < dlvList.size(); index++){
        dlvElement = (Element) dlvList.get(index);
        setAttributesForElement(konfigurationHashMap, dlvElement, index);
      }
      debugKonfigurationHashMap(konfigurationHashMap);
    } catch (Exception e) {
        tlgModuleLogger.error("getKonfigurationHashMap:" + e.getMessage());
    }
    return konfigurationHashMap;
  }
  
  public static void debugKonfigurationHashMap(HashMap <String, String> konfigurationHashMap){
    for(String key : konfigurationHashMap.keySet()){
        String debug = key+":"+konfigurationHashMap.get(key);
        if((key.startsWith("DLV") && key.endsWith("_A")) || (key.startsWith("DPD") && key.endsWith("_B")) && konfigurationHashMap.get(key) != null)
          debug += "->int:"+hextoInt(konfigurationHashMap.get(key));
      
        tlgModuleLogger.info("debugKonfigurationHashMap:" + debug);
    }
  }
  
  @SuppressWarnings ("unchecked")
  public static void setAttributesForElement (HashMap <String, String> konfigurationHashMap, Element element, Integer ordernumber) {
    List <Attribute> elementAttributes = (List <Attribute>) element.getAttributes();
    Attribute attribute = null;
    for(int index = 0; index < elementAttributes.size(); index++){
      attribute = elementAttributes.get(index);
      if(attribute.getValue() != null && attribute.getValue().length() > 0){
        if(ordernumber == null)
          konfigurationHashMap.put(element.getName()+"_"+attribute.getName(), attribute.getValue());
        else
          konfigurationHashMap.put(element.getName()+ordernumber+"_"+attribute.getName(), attribute.getValue());
      }else{
        if(ordernumber == null)
          konfigurationHashMap.put(element.getName()+"_"+attribute.getName(),null);
        else
          konfigurationHashMap.put(element.getName()+ordernumber+"_"+attribute.getName(),null);
      }
     
    }
  }
  
  public static Integer hextoInt(String hexString){
    return Integer.parseInt(hexString,16);
  }
  
  public static Document openDocument (String url) throws JDOMException {
    Document doc = null;
    SAXBuilder builder = new SAXBuilder();
    try {
      doc = builder.build (url);
    } catch (Exception e) {
        tlgModuleLogger.error("openDocument:" + e.getMessage());
    }
    return doc;
  }
  
  public static Double getPositionValue(int position){
    return position * 0.0000001;
  }
  
  public static Double getDopValue(int dop){
    return dop * 0.1;
  }
  
  public static Date getCalendarForDaysAndMillisecondsSince1980(Integer daysSince1980, Long millisecondsSinceMidnight){
    Calendar cal = new GregorianCalendar(1980,Calendar.JANUARY,1);
    if(daysSince1980 != null){
      cal.add(Calendar.DATE,daysSince1980);
    }
    if(millisecondsSinceMidnight != null){
      cal.add(Calendar.HOUR_OF_DAY,(int)(millisecondsSinceMidnight / (60*60*1000)));
      cal.add(Calendar.MINUTE,(int)((millisecondsSinceMidnight % (60*60*1000)) / (60*1000)));
      cal.add(Calendar.SECOND, (int)(((millisecondsSinceMidnight % (60*60*1000)) % (60*1000)) / 1000));
      cal.add(Calendar.MILLISECOND, (int)(((millisecondsSinceMidnight % (60*60*1000)) % (60*1000)) % 1000));
    }
    return cal.getTime();
  }
  
  public static String formatDateToString(Date date){
    SimpleDateFormat sd = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
    return sd.format(date);
  }
  
  public static Date formatStringToDate(String date){
    SimpleDateFormat sd = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
    try {
      return sd.parse(date);
    } catch (ParseException e) {
      tlgModuleLogger.error("formatStringToDate:" + e.getMessage());
      return null;
    }
  }
  
  public static boolean fileExist(String fileName){
    File f = new File(fileName);
    return f.exists();
  }
  
  public static String getPackageClassName(){
    return TLGModule.class.getName();
  }
}
