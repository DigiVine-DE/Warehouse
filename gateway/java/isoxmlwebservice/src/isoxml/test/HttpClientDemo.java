package isoxml.test;

import java.io.BufferedReader;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.File;
import java.io.FileOutputStream;
import java.io.FileReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStreamWriter;
import java.net.URL;
import java.net.URLConnection;

/**
 * Test for Rest-Configuration
 * TODO: test
 *
 */
public class HttpClientDemo {
  private static String tmpDirectory = "/tmp/";
  private static String isoXmlTaskDatei = tmpDirectory + "TASKDATA.XML";
  private static String geoJSONDatei = tmpDirectory + "order.geojson";
  
  private static String checkIsoXMLConformRequest = tmpDirectory + "CheckIsobusXMLConformRequest.xml";
  private static String checkIsoXMLConformResponse = tmpDirectory + "CheckIsobusXMLConformResponse.result";
  
  private static String convertTaskDataToGeojsonResponse = tmpDirectory + "ConvertTaskDataToGeojsonResponse.geojson";
  
  private static String convertGeosonToTaskdataResponse = tmpDirectory + "ConvertGeosonToTaskdataResponse.xml";
  
  public static void readDateiToStream(URLConnection conn, String inputFile)throws Exception{
    String data = getStringfromXMLFile(inputFile);
    conn.setDoOutput(true); 
    OutputStreamWriter wr = new OutputStreamWriter(conn.getOutputStream()); 
    wr.write(data); 
    wr.flush(); 
  }
  
  public static void writeDateiToFile(URLConnection conn,String outputFile)throws Exception{
    DataOutputStream destination = null;  
    DataInputStream source = null;  
    File outputFileAsFile = new File(outputFile); 
    FileOutputStream fos = new FileOutputStream(outputFileAsFile); 
    destination = new DataOutputStream(fos);
    InputStream urlconninstr = conn.getInputStream();
    source = new DataInputStream(urlconninstr); 
    int len = 0;
    byte[] bbuf = new byte[1024];
    while((len = source.read(bbuf)) != -1) {
      destination.write(bbuf, 0, len);
    }
    source.close();
    destination.flush();
    destination.close();
  }
  
  public static void main(String args[]) {
       
        try {   
          URL url = new URL("http://localhost:8080/isoxmlservice/command/task/check_taskdata/debug");
          URLConnection conn = url.openConnection();
          readDateiToStream(conn, isoXmlTaskDatei);
          writeDateiToFile(conn, checkIsoXMLConformRequest);
    
        } catch (Exception e) {
          e.printStackTrace();
          
        } 
        
        try {   
          URL url = new URL("http://localhost:8080/isoxmlservice/command/task/check_taskdata");
          URLConnection conn = url.openConnection();
          readDateiToStream(conn, isoXmlTaskDatei);
          writeDateiToFile(conn, checkIsoXMLConformResponse);
    
        } catch (Exception e) {
          e.printStackTrace();
          
        }
    
          try {   
            URL url = new URL("http://localhost:8080/isoxmlservice/command/task/convert_taskdata_to geojson");
            URLConnection conn = url.openConnection();
            readDateiToStream(conn, isoXmlTaskDatei);
            writeDateiToFile(conn, convertTaskDataToGeojsonResponse);
        
          } catch (Exception e) {
            e.printStackTrace();
            
          } 
          
         
          try {   
            URL url = new URL("http://localhost:8080/isoxmlservice/command/task/convert_geoson_to_taskdata");
            URLConnection conn = url.openConnection();
            readDateiToStream(conn, geoJSONDatei);
            writeDateiToFile(conn, convertGeosonToTaskdataResponse);
        
          } catch (Exception e) {
            e.printStackTrace();
            
          } 
  }
  
  public static String getStringfromXMLFile(String dateiName){
    String content = "";
    BufferedReader br = null;
    try {
      String thisLine = null;
      br = new BufferedReader(new FileReader(dateiName));
      while ((thisLine = br.readLine()) != null) { 
        content +=thisLine+"\n";
      }
    } 
    catch ( IOException e ) { 
      System.err.println( "mistake handling " + dateiName ); 
      System.exit( 1 ); 
    }
    return content;
  }
}
