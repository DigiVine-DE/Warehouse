package isoxml.tlg;

import java.io.DataInputStream;
import java.io.EOFException;
import java.io.FileInputStream;
import java.io.IOException;
import java.math.BigDecimal;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.Date;
import java.util.LinkedHashMap;
import java.util.Vector;

import org.apache.log4j.Logger;

import isoxml.iso11783.DLV;
import isoxml.iso11783.PTN;

/**
 * Reads TLG BIN Files
 *
 */
public class TLGBinReader {
  private static Logger binReaderLogger = Logger.getLogger(TLGBean.class.getName());  

  /**
   * @param args
   */
  public static void main (String[] args) {
    readBinaryFileUpperLower("/tmp/", "TLG00001.bin", "/tmp/", "TLG00001.xml", true, false);
  }
  
  public static void readBinaryFileUpperLower(String logFileDirectory, String logFileName, String  xmlFileDirectory, String xmlFileName, boolean isByteStructureBigEndian, boolean debug){
    try{
      if(TLGModule.fileExist(logFileDirectory+logFileName) && TLGModule.fileExist(xmlFileDirectory+xmlFileName))
        readBinaryFile(logFileDirectory+logFileName, xmlFileDirectory+xmlFileName, isByteStructureBigEndian,debug);
      else if(TLGModule.fileExist(logFileDirectory+logFileName.toUpperCase()) && TLGModule.fileExist(xmlFileDirectory+xmlFileName.toUpperCase()))
        readBinaryFile(logFileDirectory+logFileName.toUpperCase(), xmlFileDirectory+xmlFileName.toUpperCase(), isByteStructureBigEndian,debug);
    }catch(Exception e){
        binReaderLogger.error("readBinaryFileUpperLower:" + e.getMessage());
    }
  }
  
  public static Vector <TLGBean> readBinaryFile(String logFileName, String  xmlFileName, boolean isByteStructureBigEndian, boolean debug){
    LinkedHashMap <String, String> konfigurationHashMap = null;
    DataInputStream dis = null;
    Vector <TLGBean> tlgVector = new Vector <TLGBean> ();
    TLGBean tlgBean = null;
    try {
      konfigurationHashMap = TLGModule.getKonfigurationHashMap(xmlFileName);
      dis = new DataInputStream(new FileInputStream(logFileName));
      int index = 0;
      while(true){
        tlgBean = readBinaryFileEntry(dis, konfigurationHashMap, isByteStructureBigEndian, debug);
        if(tlgBean != null)
          tlgVector.add(tlgBean);
        index++;
        if((index%1000) == 0){
            binReaderLogger.debug("readBinaryFile:1000-Package " + (index/1000) + " loaded.");
        }
      }
    } catch (EOFException eof) {
        binReaderLogger.info("readBinaryFile:Reached end of file!");
    } catch (Exception e) {
        binReaderLogger.error("readBinaryFileUpperLower:" + e.getMessage());
    }
    return tlgVector;
  }
  
  public static TLGBean readBinaryFileEntry(DataInputStream dis, LinkedHashMap <String, String> konfigurationHashMap, boolean isByteStructureBigEndian, boolean debug) throws EOFException{
    TLGBean tlgBean = new TLGBean();
    try {
      //TIM
      Long millisecondsSinceMidnight = null;
      Integer daysSince1980 = null;
      Date calendarDatum = null;
      //PTN
      Integer northDegrees = null;
      Integer eastDegrees = null;
      Integer pdopQuality = null;
      Integer hdopQuality = null;
      Long millisecondsSinceMidnightUTC = null;
      Integer daysSince1980UTC = null;
      Date calendarDatumUTC = null;
      
      //TIM, A
      if(konfigurationHashMap.containsKey("TIM_A")){
        if(konfigurationHashMap.get("TIM_A") == null){
          //Uhrzeit
          millisecondsSinceMidnight = bit32ToUnsignedLong(intToLittleEndianIfNecessary(dis.readInt(), isByteStructureBigEndian));
          //Datum
          daysSince1980 = bit16ToUnsignedInt(shortToLittleEndianIfNecessary(dis.readShort(), isByteStructureBigEndian));
          //Calendar-Formatiert
          calendarDatum = TLGModule.getCalendarForDaysAndMillisecondsSince1980(daysSince1980, millisecondsSinceMidnight);
        }else{
          calendarDatum = TLGModule.formatStringToDate(konfigurationHashMap.get("TIM_A"));
        }
      }
      tlgBean.setCalendarDatum(calendarDatum);
      
      PTN ptn = new PTN();
      //PTN, A
      if(konfigurationHashMap.containsKey("PTN_A")){
        if(konfigurationHashMap.get("PTN_A") == null){
          northDegrees = Integer.valueOf(intToLittleEndianIfNecessary(dis.readInt(), isByteStructureBigEndian));
          ptn.setA(BigDecimal.valueOf(TLGModule.getPositionValue(northDegrees)));
        }else{
          ptn.setA(BigDecimal.valueOf(Double.valueOf(konfigurationHashMap.get("PTN_A"))));
        }
      }
      
      //PTN, B
      if(konfigurationHashMap.containsKey("PTN_B")){
        if(konfigurationHashMap.get("PTN_B") == null){
          eastDegrees = Integer.valueOf(intToLittleEndianIfNecessary(dis.readInt(), isByteStructureBigEndian));
          ptn.setB(BigDecimal.valueOf(TLGModule.getPositionValue(eastDegrees)));
        }else{
          ptn.setB(BigDecimal.valueOf(Double.valueOf(konfigurationHashMap.get("PTN_B"))));
        }
      }
      
      //PTN, C
      if(konfigurationHashMap.containsKey("PTN_C")){
        if(konfigurationHashMap.get("PTN_C") == null){
          ptn.setC(Integer.valueOf(intToLittleEndianIfNecessary(dis.readInt(), isByteStructureBigEndian)));
        }else{
          ptn.setC(Integer.valueOf(konfigurationHashMap.get("PTN_C")));
        }
      }
     
      //PTN, D
      if(konfigurationHashMap.containsKey("PTN_D")){
        if(konfigurationHashMap.get("PTN_D") == null){
          ptn.setD(String.valueOf(bit8ToUnsignedInt(dis.readByte())));
        }else{
          ptn.setD(konfigurationHashMap.get("PTN_D"));
        }
      }
      
      //PTN, E
      if(konfigurationHashMap.containsKey("PTN_E")){
        if(konfigurationHashMap.get("PTN_E") == null){
          pdopQuality = Integer.valueOf(bit16ToUnsignedInt(shortToLittleEndianIfNecessary(dis.readShort(), isByteStructureBigEndian)));
          ptn.setE(BigDecimal.valueOf(TLGModule.getDopValue(pdopQuality)));
        }else{
          ptn.setE(BigDecimal.valueOf(Double.valueOf(konfigurationHashMap.get("PTN_E"))));
        }
      }
      
      //PTN, F
      if(konfigurationHashMap.containsKey("PTN_F")){
        if(konfigurationHashMap.get("PTN_F") == null){
          hdopQuality = Integer.valueOf(bit16ToUnsignedInt(shortToLittleEndianIfNecessary(dis.readShort(), isByteStructureBigEndian)));
          ptn.setF(BigDecimal.valueOf(TLGModule.getDopValue(hdopQuality)));
        }else{
          ptn.setF(BigDecimal.valueOf(Double.valueOf(konfigurationHashMap.get("PTN_F"))));
        }
      }
      
      //PTN, G
      if(konfigurationHashMap.containsKey("PTN_G")){
        if(konfigurationHashMap.get("PTN_G") == null){
          ptn.setG(Integer.valueOf(bit8ToUnsignedInt(dis.readByte())).shortValue());
        }else{
          ptn.setG(Integer.valueOf(konfigurationHashMap.get("PTN_G")).shortValue());
        }  
      }
      
      //PTN, H
      if(konfigurationHashMap.containsKey("PTN_H")){
        if(konfigurationHashMap.get("PTN_H") == null){
          ptn.setH(Long.valueOf(bit32ToUnsignedLong(intToLittleEndianIfNecessary(dis.readInt(), isByteStructureBigEndian))));
        }else{
          ptn.setH(Long.valueOf(millisecondsSinceMidnightUTC = Long.parseLong(konfigurationHashMap.get("PTN_H"))));
        }   
      }
      
      //PTN, I
      if(konfigurationHashMap.containsKey("PTN_I")){
        if(konfigurationHashMap.get("PTN_I") == null){
          ptn.setI(Integer.valueOf(bit16ToUnsignedInt(shortToLittleEndianIfNecessary(dis.readShort(), isByteStructureBigEndian))));
        }else{
          ptn.setI(Integer.valueOf(daysSince1980UTC = Integer.parseInt(konfigurationHashMap.get("PTN_I"))));
        }  
      }
      tlgBean.setPtn(ptn);
      
      //PTN, H && PTN, I
      if(konfigurationHashMap.containsKey("PTN_H") && konfigurationHashMap.containsKey("PTN_I")){
        calendarDatumUTC = TLGModule.getCalendarForDaysAndMillisecondsSince1980(daysSince1980UTC, millisecondsSinceMidnightUTC);
      }
      tlgBean.setCalendarDatumUTC(calendarDatumUTC);
      
      //#DLV
      Integer numberDLVs = bit8ToUnsignedInt(dis.readByte());
      tlgBean.setNumberDLVs(numberDLVs);
      
      //DLVs
      LinkedHashMap <Integer, DLV> dlvLinkedHashMap = readDLVs(dis, numberDLVs, isByteStructureBigEndian, konfigurationHashMap);
      tlgBean.setDlvLinkedHashMap(dlvLinkedHashMap);
      
      if(debug == true){
        tlgBean.debug();
      }
    } catch (EOFException eof) {
      tlgBean = null;
      throw new EOFException();
    } catch (Exception e) {
      tlgBean = null;
      binReaderLogger.error("readBinaryFileEntry:" + e.getMessage());
    }
    return tlgBean;
  }
  
  public static LinkedHashMap <Integer, DLV> readDLVs(DataInputStream dis, int numberDLVs, boolean isByteStructureBigEndian, LinkedHashMap <String, String> konfigurationHashMap){
    LinkedHashMap <Integer, DLV> dlvLinkedHashMap = new LinkedHashMap<Integer, DLV>();
    DLV dlv = null;
    for(int index = 0; index < numberDLVs; index++){
      dlv = new DLV();
      try {
        int orderingNumber = bit8ToUnsignedInt(dis.readByte());
        
        //DLV, A
        if(konfigurationHashMap.containsKey("DLV"+orderingNumber+"_A")){
          dlv.setA(konfigurationHashMap.get("DLV"+orderingNumber+"_A").getBytes());
        }
        
        //DLV, B
        if(konfigurationHashMap.containsKey("DLV"+orderingNumber+"_B")){
          if(konfigurationHashMap.get("DLV"+orderingNumber+"_B") == null){
            dlv.setB(Integer.valueOf(intToLittleEndianIfNecessary(dis.readInt(), isByteStructureBigEndian)));
          }else{
            dlv.setB(Integer.valueOf(konfigurationHashMap.get("DLV"+orderingNumber+"_B")));
          }
        }
        
        //DLV, C
        if(konfigurationHashMap.containsKey("DLV"+orderingNumber+"_C")){
          if(konfigurationHashMap.get("DLV"+orderingNumber+"_C") == null){
            dlv.setC("DET-0");
          }else{
            dlv.setC(konfigurationHashMap.get("DLV"+orderingNumber+"_C"));
          }
        }
        
        dlvLinkedHashMap.put(TLGModule.hextoInt(new String(dlv.getA())), dlv);
      } catch (IOException e) {
          binReaderLogger.error("readDLVs:" + e.getMessage());
      }
    }
    return dlvLinkedHashMap;
  }
  
  public static String showDLVs(DataInputStream dis, int numberDLVs, boolean isByteStructureBigEndian){
    String dlvString = "";
    for(byte index = 0; index < numberDLVs; index++){
      try {
        int dlvOrderingNumber = bit8ToUnsignedInt(dis.readByte());
        int processDataValue = intToLittleEndianIfNecessary(dis.readInt(), isByteStructureBigEndian);
        dlvString += "DLV"+dlvOrderingNumber+":"+processDataValue+";";
      } catch (IOException e) {
          binReaderLogger.error("showDLVs:" + e.getMessage());
      }
    }
    return dlvString;
  }
  
  public static long longToLittleEndianIfNecessary(long value, boolean isByteStructureBigEndian) {
    if(isByteStructureBigEndian == true){
      ByteBuffer buf = ByteBuffer.allocate(8);  
      buf.order(ByteOrder.BIG_ENDIAN);  
      buf.putLong(value);  
      buf.order(ByteOrder.LITTLE_ENDIAN);  
      return buf.getLong(0);
    }else{
      return value;
    }
  }  
  
  public static int intToLittleEndianIfNecessary(int value, boolean isByteStructureBigEndian) {
    if(isByteStructureBigEndian == true){
      ByteBuffer bbuf = ByteBuffer.allocate(4);  
      bbuf.order(ByteOrder.BIG_ENDIAN);  
      bbuf.putInt(value);  
      bbuf.order(ByteOrder.LITTLE_ENDIAN);  
      return bbuf.getInt(0);
    }else{
      return value;
    }
  }  
  
  public static short shortToLittleEndianIfNecessary(short value, boolean isByteStructureBigEndian) {
    if(isByteStructureBigEndian == true){
      ByteBuffer bbuf = ByteBuffer.allocate(2);  
      bbuf.order(ByteOrder.BIG_ENDIAN);  
      bbuf.putShort(value);  
      bbuf.order(ByteOrder.LITTLE_ENDIAN);  
      return bbuf.getShort(0);
    }else{
      return value;
    }
  }
  
  public static double doubleToLittleEndianIfNecessary(double value, boolean isByteStructureBigEndian) {
    if(isByteStructureBigEndian == true){
      ByteBuffer bbuf = ByteBuffer.allocate(8);  
      bbuf.order(ByteOrder.BIG_ENDIAN);  
      bbuf.putDouble(value);  
      bbuf.order(ByteOrder.LITTLE_ENDIAN);  
      return bbuf.getDouble(0);
    }else{
      return value;
    }
  }
  
  public static float floatToLittleEndianIfNecessary(float value, boolean isByteStructureBigEndian) {  
    if(isByteStructureBigEndian == true){
      ByteBuffer bbuf = ByteBuffer.allocate(4);  
      bbuf.order(ByteOrder.BIG_ENDIAN);  
      bbuf.putFloat(value);  
      bbuf.order(ByteOrder.LITTLE_ENDIAN);  
      return bbuf.getFloat(0);
    }else{
      return value;
    }
  } 
  
  public static int bit8ToUnsignedInt( byte b )
  {
    return Integer.valueOf(b & 0xFF);
  }
  
  public static int bit16ToUnsignedInt( short b )
  {
    return Integer.valueOf(b & 0xFFFF);
  }
  
  public static long bit32ToUnsignedLong( int b )
  {
    return Long.valueOf(b & 0xFFFFFFFFL);
  }
}
