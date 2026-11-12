package isoxml.tlg;

import java.util.Date;
import java.util.LinkedHashMap;

import org.apache.log4j.Logger;

import isoxml.iso11783.DLV;
import isoxml.iso11783.PTN;

/**
 * Bean for TLG
 *
 */
public class TLGBean {
  private static Logger tlgBeanLogger = Logger.getLogger(TLGBean.class.getName());  
    
  private Integer id = null;
  private Date calendarDatum = null;
  private Date calendarDatumUTC = null;
  private Integer numberDLVs = null;
  private PTN ptn = null;
  private LinkedHashMap <Integer, DLV> dlvLinkedHashMap = null;
  
  public TLGBean() {
  }
  
  public Integer getId () {
    return id;
  }

  public void setId (Integer id) {
    this.id = id;
  }

  public Date getCalendarDatum () {
    return calendarDatum;
  }

  public void setCalendarDatum (Date calendarDatum) {
    this.calendarDatum = calendarDatum;
  }

  public Date getCalendarDatumUTC () {
    return calendarDatumUTC;
  }

  public void setCalendarDatumUTC (Date calendarDatumUTC) {
    this.calendarDatumUTC = calendarDatumUTC;
  }

  public Integer getNumberDLVs () {
    return numberDLVs;
  }

  public void setPtn (PTN ptn) {
    this.ptn = ptn;
  }

  public PTN getPtn () {
    return ptn;
  }

  public void setNumberDLVs (Integer numberDLVs) {
    this.numberDLVs = numberDLVs;
  }

  public LinkedHashMap<Integer, DLV> getDlvLinkedHashMap () {
    return dlvLinkedHashMap;
  }

  public void setDlvLinkedHashMap (LinkedHashMap<Integer, DLV> dlvLinkedHashMap) {
    this.dlvLinkedHashMap = dlvLinkedHashMap;
  }

  public void debug(){
    String content1 = "";
    if(calendarDatum != null)
      content1+= calendarDatum;
    if(ptn.getA() != null)
      content1+= " " + ptn.getA();
    if(ptn.getB() != null)
      content1+= " " + ptn.getB();
    if(ptn.getC() != null)
      content1+= " " + ptn.getC();
    if(ptn.getD() != null)
      content1+= " " + ptn.getD();
    if(ptn.getE() != null)
      content1+= " " + ptn.getE();
    if(ptn.getF() != null)
      content1+= " " + ptn.getF();
    if(ptn.getG() != null)
      content1+= " " + ptn.getG();
    if(calendarDatumUTC != null)
      content1+= " " + calendarDatumUTC;
    if(numberDLVs != null)
      content1+= " " + numberDLVs;
    tlgBeanLogger.info("debug:" + content1);
    
    
    if(dlvLinkedHashMap != null){
      for(Integer key : dlvLinkedHashMap.keySet()){
        DLV tempDLV = dlvLinkedHashMap.get(key);
        String content2 = "intDDi:" + key
                        +",hexDDI:" + new String(tempDLV.getA())
                        +",value:" + tempDLV.getB()
                        +",idref:" + tempDLV.getC();
        tlgBeanLogger.info("debugAusgabe:" + content2);
      }
    }
  }
}
