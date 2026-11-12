package isoxml.tlg;

import java.io.FileReader;
import java.util.LinkedHashMap;

import org.apache.log4j.Logger;

import au.com.bytecode.opencsv.CSVReader;


/**
 * reads ddi csv
 *
 */
public class DDIReader {
  private static Logger ddiReaderLogger = Logger.getLogger(TLGBean.class.getName());  
  
  @SuppressWarnings("unused")
  public static void main (String[] args) {
    LinkedHashMap <String, DDIBean> ddiLinkedHashMap = readDDIFile("/tmp/ddi_2011.csv");
  }
  
  public static LinkedHashMap <String, DDIBean> readDDIFile(String fileName){
    LinkedHashMap <String, DDIBean> ddiLinkedHashMap = new LinkedHashMap <String, DDIBean>();
    try {
      CSVReader reader = new CSVReader(new FileReader(fileName), ',', '"');
      DDIBean ddiBean = null;
      String [] csvZeileAlsStringArray = null;
      while ((csvZeileAlsStringArray = reader.readNext()) != null) {
        ddiBean = createDDIBeanFromStringBuffer(csvZeileAlsStringArray);
        ddiLinkedHashMap.put(ddiBean.getDdIdentifier(), ddiBean);
      }
    } catch (Exception e) {
        ddiReaderLogger.error("readDDIFile:" + e.getMessage());
    }
    return ddiLinkedHashMap;
  }
  
  public static DDIBean createDDIBeanFromStringBuffer(String [] csvZeileAlsStringArray){
    DDIBean ddiBean = new DDIBean();
    try{
      ddiBean.setDdIdentifier(csvZeileAlsStringArray[0]);
      ddiBean.setDdeName(csvZeileAlsStringArray[1]);
      ddiBean.setDefinition(csvZeileAlsStringArray[2]);
      ddiBean.setComment(csvZeileAlsStringArray[3]);
      ddiBean.setDeviceClassses(csvZeileAlsStringArray[4]);
      ddiBean.setUnitSymbol(csvZeileAlsStringArray[5]);
      ddiBean.setUnitQuantity(csvZeileAlsStringArray[6]);
      ddiBean.setBitResolution(csvZeileAlsStringArray[7]);
      ddiBean.setSaespn(csvZeileAlsStringArray[8]);
      ddiBean.setMinimumValue(csvZeileAlsStringArray[9]);
      ddiBean.setMaximumValue(csvZeileAlsStringArray[10]);
      ddiBean.setSubmitBy(csvZeileAlsStringArray[11]);
      ddiBean.setSubmitDate(csvZeileAlsStringArray[12]);
      ddiBean.setSubmitCompany(csvZeileAlsStringArray[13]);
      ddiBean.setStatus(csvZeileAlsStringArray[14]);
      ddiBean.setStatusDate(csvZeileAlsStringArray[15]);
      ddiBean.setRequestDocument(csvZeileAlsStringArray[16]);
      ddiBean.setAnnexReleaseDate(csvZeileAlsStringArray[17]);
      ddiBean.setStatusCommentsInteger(csvZeileAlsStringArray[18]);
    }catch(Exception e){
        ddiReaderLogger.error("createDDIBeanFromStringBuffer:" + e.getMessage());
    }
    return ddiBean;
  }
}
