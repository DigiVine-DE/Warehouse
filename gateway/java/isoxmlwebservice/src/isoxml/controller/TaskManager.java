package isoxml.controller;

import java.io.BufferedReader;
import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.math.BigDecimal;
import java.net.HttpURLConnection;
import java.net.MalformedURLException;
import java.net.URL;
import java.util.ArrayList;
import java.util.Calendar;
import java.util.GregorianCalendar;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Properties;
import java.util.TimeZone;

import jakarta.servlet.ServletOutputStream;
import jakarta.servlet.http.HttpServlet;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import jakarta.xml.bind.JAXBException;
import jakarta.xml.bind.DatatypeConverter;

import org.apache.commons.io.FileUtils;
import org.apache.log4j.Logger;
import org.geojson.Feature;
import org.geojson.GeoJsonObject;
import org.geojson.LngLatAlt;
import org.geojson.Polygon;
import org.xml.sax.SAXException;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import isoxml.bean.TaskInformation;
import isoxml.bean.TaskInformationPlantProtection;
import isoxml.bean.TaskInformationPoint;
import isoxml.exception.FileException;
import isoxml.exception.ManagerException;
import isoxml.exception.XMLException;
import isoxml.iso11783.CTP;
import isoxml.iso11783.CTR;
import isoxml.iso11783.CVT;
import isoxml.iso11783.DPT;
import isoxml.iso11783.DVC;
import isoxml.iso11783.FRM;
import isoxml.iso11783.ISO11783TaskData;
import isoxml.iso11783.LSG;
import isoxml.iso11783.PDT;
import isoxml.iso11783.PDV;
import isoxml.iso11783.PFD;
import isoxml.iso11783.PGP;
import isoxml.iso11783.PLN;
import isoxml.iso11783.PNT;
import isoxml.iso11783.TSK;
import isoxml.iso11783.TZN;
import isoxml.iso11783.VPN;
import isoxml.utilities.ImportExportUtilities;
import isoxml.utilities.JaxbMarshalUnmarshalUtil;
import isoxml.utilities.RequestReader;
import isoxml.utilities.XMLParser;


public class TaskManager extends HttpServlet implements ManagerInterface {
    
    private static final long serialVersionUID = -5180659592296153641L;
    private Properties applikationsParameter;
    private Object tmpDir;
    private final String workHarvest = "harvest";
    private final String workSpray = "spray";
    private final String plantProtectionUnitLiter = "l/ha";
    private final String plantProtectionUnitGramm = "g/ha";
    private static Integer tlgFilename = Integer.valueOf(0);
    
    private static Logger taskManagerLogger = Logger.getLogger(TaskManager.class.getName());
        
    public TaskManager() {
    }

    @Override
    public void init(java.util.Properties properties, Object tmpDir) throws ManagerException {
        this.applikationsParameter = properties;
        this.tmpDir = tmpDir;
    }
    
    /**
     * Endpoints API TAsk-Manager
     *
     */
    @Override
    public void processRequest(HttpServletRequest request, HttpServletResponse response, String prefix) throws ManagerException {
        taskManagerLogger.info("processRequest");
        String absoluteFilename = null;
        HashMap<String, String> parameterMap = null;
        
        String work = RequestReader.getSubAction(request).toLowerCase();
        String command = RequestReader.getSubSubAction(request).toLowerCase();
        
        try {
            parameterMap = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/DataWareHouse");
        } catch (XMLException e) {
            taskManagerLogger.error("processRequest:" + e.getMessage());
        }
        
        Boolean activateDataStorage = Boolean.valueOf(parameterMap.get("activateDataStorage"));
        String datawareHouseToDoUrl = null;
        String datawareHouseResultUrl = null;
        
        switch(work){
        case workHarvest:
            datawareHouseToDoUrl = parameterMap.get("postTaskHarvestToDoURL");
            datawareHouseResultUrl = parameterMap.get("postTaskHarvestResultURL");
            break;
        case workSpray:
            datawareHouseToDoUrl = parameterMap.get("postTaskSprayToDoURL");
            datawareHouseResultUrl = parameterMap.get("postTaskSprayResultURL");
            break;
        }
        
        //http://127.0.0.1:8080/isoxmlwebservice/command/task/<work>/<command>?hashkey=<hashkey>
        if(command.equals("check_taskdata")) {
            isoxmltaskschemaconform(request, response, prefix); 
        } else if (command.equals("convert_geojson_to_taskdata")) {
            absoluteFilename = createISOXMLTask(request, response, prefix, work, activateDataStorage, datawareHouseToDoUrl);
        } else if (command.equals("convert_taskdata_to_geojson")) {
            absoluteFilename = createGeoJSONTask(request, response, prefix, work, activateDataStorage, datawareHouseResultUrl); 
        }
        if(absoluteFilename != null && absoluteFilename.length() > 0) {
            sendHttpServletResponseOrURL(absoluteFilename, response, null, "application/zip");
        }
    }
    
    /**
     * Endpoints API TAsk-Manager check_taskdata
     * check if TASKDATA is valid
     *
     */
    private void isoxmltaskschemaconform(HttpServletRequest request, HttpServletResponse response, String prefix) {
        taskManagerLogger.info("isoxmltaskschemaconform->function");
       
        String result = "mistake";
        String subcommand = RequestReader.getSubSubAction(request).toLowerCase();
        
        ImportExportUtilities ieu = new ImportExportUtilities(tmpDir);
        Boolean copySuccessInputStream = Boolean.valueOf(true);
        FileInputStream fileInputStream = null;
        String taskDataFilename = null;
        try {
            HashMap<String, String> parameterMap = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/ISOXML");
            taskDataFilename = parameterMap.get("taskDataFilename");
            String importDirectory = ieu.createFilesFromImport(request.getInputStream());
            File taskDataFile = new File(importDirectory + File.separator + taskDataFilename);
            fileInputStream = new FileInputStream(taskDataFile);
        } catch (IOException e) {
            copySuccessInputStream = Boolean.valueOf(false);
            taskManagerLogger.error("isoxmltaskschemaconform->InputStream can't becopied.->" + e.getMessage());
        } catch (XMLException e) {
            copySuccessInputStream = Boolean.valueOf(false);
            taskManagerLogger.error("isoxmltaskschemaconform->Konfiguration-Xml can't be parsed.->" + e.getMessage());
        } 

        if (copySuccessInputStream.booleanValue() == true && subcommand.equals("debug")) {
            try {
                createPlainTextToDownload(request, response, taskDataFilename, RequestReader.convertStreamToString(fileInputStream));
                result = "send request text back";
            } catch (IOException e) {
                taskManagerLogger.error("isoxmltaskschemaconform->Request-String can't be published (1).->" + e.getMessage());
            } catch (FileException e) {
                taskManagerLogger.error("isoxmltaskschemaconform->Request-String can't be published (2).->" + e.getMessage());
            } finally {
                try {
                    fileInputStream.close();
                    ieu.deleteImportFiles();
                } catch (IOException e) {
                    taskManagerLogger.error("isoxmltaskschemaconform->fileInputStream can't be closed.->" + e.getMessage());
                }
            }
        } else if (copySuccessInputStream.booleanValue() == true && !subcommand.equals("debug")) {
            try {
                HashMap<String, String> parameterMap = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/ISOXML");
                JaxbMarshalUnmarshalUtil.unmarshalwebservice(parameterMap.get("isoxmlTaskXsdDatei"), fileInputStream, ISO11783TaskData.class);
                result = "isoxmlconform";
            } catch (JAXBException e) {
                result = "!isoxmlconform";
                taskManagerLogger.error("isoxmltaskschemaconform->Request-String doesn't match  XSD(JAXBException).->" + e.getMessage());
            } catch (SAXException e) {
                result = "!isoxmlconform";
                taskManagerLogger.error("isoxmltaskschemaconform->Request-String doesn't match  XSD(SAXException).->" + e.getMessage());
            } catch (XMLException e) {
                result = "!isoxmlconform";
                taskManagerLogger.error("isoxmltaskschemaconform->Request-String doesn't match  XSD(XMLException).->" + e.getMessage());
            } finally {
                try {
                    fileInputStream.close();
                    ieu.deleteImportFiles();
                } catch (IOException e) {
                    taskManagerLogger.error("isoxmltaskschemaconform->fileInputStream can't be closed.->" + e.getMessage());
                }
            }
            try {
                createPlainTextToDownload(request, response, taskDataFilename, result);
            } catch (FileException e) {
                taskManagerLogger.error("isoxmltaskschemaconform->conform/nicht conform konnte nicht zurückgegeben werden->" + e.getMessage());
            }
        } else {
            taskManagerLogger.error("isoxmltaskschemaconform->Ist der Anfragestring vielleicht leer?");
        }
    }
    
    /**
     * Endpoints API TAsk-Manager convert_geoson_to_taskdata
     * create TASKDATA from GeoJSON (FMIS)
     *
     */
    private String createISOXMLTask(HttpServletRequest request, HttpServletResponse response, String prefix, String work, Boolean activateDataStorage, String datawareHouseUrl) {
        taskManagerLogger.info("createISOXMLTask->function");
        String absoluteTaskDataFilenameZip = null;
        ImportExportUtilities ieu = new ImportExportUtilities(tmpDir);
        FileInputStream fileInputStream = null;
        try {
            HashMap<String, String> parameterMapGeoJSON = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/GeoJSON");
            String geoJsonFilename = parameterMapGeoJSON.get("geoJsonFilename");
            HashMap<String, String> parameterMapISOXML = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/ISOXML");
            String taskDataFilename = parameterMapISOXML.get("taskDataFilename");
            
            String workDirectory = ieu.createFilesFromImport(request.getInputStream());
            File geoJsonFile = new File(workDirectory + File.separator + geoJsonFilename);
            fileInputStream = new FileInputStream(geoJsonFile);
            
            Feature feature = new ObjectMapper().readValue(fileInputStream, Feature.class);
            
            TaskInformation tI = new TaskInformation();
            tI.setVersionMajor(parameterMapISOXML.get("versionMajor"));
            tI.setVersionMinor(parameterMapISOXML.get("versionMinor"));
            tI.setManagementSoftwareManufacturer(parameterMapISOXML.get("managementSoftwareManufacturer"));
            tI.setManagementSoftwareVersion(parameterMapISOXML.get("managementSoftwareVersion"));
            tI.setDataTransferOrigin(parameterMapISOXML.get("dataTransferOrigin"));
            
            tI.setTaskID(Long.valueOf(feature.getProperty("task_id").toString()));
            tI.setTaskName(feature.getProperty("task_name"));
            tI.setCustomerName(feature.getProperty("customer_name"));
            tI.setFarmName(feature.getProperty("farm_name"));
            tI.setFruitGroupName(feature.getProperty("fruit_group_name"));
            tI.setFruitName(feature.getProperty("fruit_name"));
            tI.setPlainName(feature.getProperty("plain_name"));
            tI.setPlainSize(Long.valueOf(feature.getProperty("plain_size").toString()));
            tI.setTotalArea(feature.getProperty("total_area") != null ? 
                    Integer.valueOf(feature.getProperty("total_area").toString()) : null);
            tI.setEffectiveTotalDistance(feature.getProperty("effective_total_distance") != null ? 
                    Integer.valueOf(feature.getProperty("effective_total_distance").toString()) : null);
            tI.setEffectiveTotalTime(feature.getProperty("effective_total_distance") != null ? 
                    Integer.valueOf(feature.getProperty("effective_total_distance").toString()) : null);
            
            if(work.equals(workHarvest)) {
                tI.setYieldTotalMass(feature.getProperty("yield_total_mass") != null ? 
                        Integer.valueOf(feature.getProperty("yield_total_mass").toString()) : null);
                tI.setAverageTartaricAcid(feature.getProperty("average_tartaric_acid") != null ? 
                        Integer.valueOf(feature.getProperty("average_tartaric_acid").toString()) : null);
                tI.setAverageMalicAcid(feature.getProperty("average_malic_acid") != null ? 
                        Integer.valueOf(feature.getProperty("average_malic_acid").toString()) : null);
                tI.setAverageGlucose(feature.getProperty("average_glucose") != null ? 
                        Integer.valueOf(feature.getProperty("average_glucose").toString()) : null);
                tI.setAverageFructose(feature.getProperty("average_fructose") != null ? 
                        Integer.valueOf(feature.getProperty("average_fructose").toString()) : null);
            } else if (work.equals(workSpray)) {
                ArrayList<ObjectNode> plantProtectionList = feature.getProperty("plant_protection");
                if(plantProtectionList!= null &&  !plantProtectionList.isEmpty()) {
                    ObjectMapper mapper = new ObjectMapper();
                    for (int i = 0; i < plantProtectionList.size(); i++) {
                        TaskInformationPlantProtection tmptIPP = mapper.convertValue(plantProtectionList.get(i), new TypeReference<TaskInformationPlantProtection>() {});
                        tI.addResourceToPlantProtection(tmptIPP);
                    }
                } 
                tI.setApplicationTotalVolume(feature.getProperty("application_total_volume") != null ? 
                        Integer.valueOf(feature.getProperty("application_total_volume").toString()) : null);
            }
            
            GeoJsonObject geometry = feature.getGeometry();
            if(geometry instanceof Polygon) {
                Polygon polygon = (Polygon) geometry;
                List<List<LngLatAlt>> lngLatAltListOutside = polygon.getCoordinates();
                if(lngLatAltListOutside != null && !lngLatAltListOutside.isEmpty()) {
                    List<LngLatAlt> lngLatAltListInside = lngLatAltListOutside.get(0);
                    if(lngLatAltListInside != null && !lngLatAltListInside.isEmpty()) {
                        for (int i = 0; i < lngLatAltListInside.size(); i++) {
                            TaskInformationPoint tmptIP = new TaskInformationPoint();
                            LngLatAlt lla = lngLatAltListInside.get(i);
                            tmptIP.setLongitude(lla.getLongitude());
                            tmptIP.setLatitude(lla.getLatitude());
                            tI.addPointToCoordinates(tmptIP);
                        }
                    }
                }
            }
            ISO11783TaskData taskData = this.createTaskData(tI, work);
            Calendar calendar = new GregorianCalendar();
            calendar.setTimeZone(TimeZone.getTimeZone("CET"));
            // Debug: JaxbMarshalUnmarshalUtil.createStringAsReport(taskData);
            String absoluteTaskDataFilename = ieu.getRandomDirectoryExport() + File.separator + taskDataFilename;
            JaxbMarshalUnmarshalUtil.marshal(prefix + parameterMapISOXML.get("isoxmlTaskXsdDatei"), absoluteTaskDataFilename, taskData);
            taskManagerLogger.info("createISOXMLTask-> function: marshal" + absoluteTaskDataFilename);
            absoluteTaskDataFilenameZip = ieu.createZipFilesForExport();
            // Send to DatawareHouse
            if(activateDataStorage && datawareHouseUrl != null && datawareHouseUrl.length() > 0) {
                sendHttpServletResponseOrURL(geoJsonFile.getCanonicalPath(), null, datawareHouseUrl, "application/json");
            }
        } catch (JAXBException jbe) {
            taskManagerLogger.error("createISOXMLTask(JAXBException)->" + jbe.getMessage(), jbe);
        } catch (SAXException saxe) {
            taskManagerLogger.error("createISOXMLTask(SAXException)->" + saxe.getMessage(), saxe);
        } catch (XMLException e) {
            taskManagerLogger.error("createISOXMLTask(IOException)->" + e.getMessage(),e);
        } catch (IOException e) {
            taskManagerLogger.error("createISOXMLTask(XMLException)->" + e.getMessage(),e);
        } catch (Exception e) {
            taskManagerLogger.error("createISOXMLTask(Exception)->" + e.getMessage(),e);
        } finally {
            try {
                fileInputStream.close();
                ieu.deleteImportFiles();
            } catch (IOException e) {
                taskManagerLogger.error("createISOXMLTask->fileInputStream can't be closed.->" + e.getMessage());
            }
        }
        return absoluteTaskDataFilenameZip;
    }
    
    /**
     * Endpoints API TAsk-Manager convert_taskdata_to geojson
     * create GeoJSON (FMIS) from TASKDATA 
     *
     */
    private String createGeoJSONTask(HttpServletRequest request, HttpServletResponse response, String prefix, String work, Boolean activateDataStorage, String datawareHouseUrl) {
        taskManagerLogger.info("putisoxmltask->function");
        String absoluteGeoJSONFilenameZip = null;
        ImportExportUtilities ieu = new ImportExportUtilities(tmpDir);
        FileInputStream fileInputStream = null;
        try {
            HashMap<String, String> parameterMapGeoJSON = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/GeoJSON");
            String geoJsonFilename = parameterMapGeoJSON.get("geoJsonFilename");
            HashMap<String, String> parameterMapISOXML = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/ISOXML");
            String taskDataFilename = parameterMapISOXML.get("taskDataFilename");
            HashMap<String, String> parameterMapGeneral = XMLParser.getKonfigurationMap(applikationsParameter.getProperty("parameterFile"), "Konfiguration/General");
            String charset = parameterMapGeneral.get("charset");
            
            String workDirectory = ieu.createFilesFromImport(request.getInputStream());
            File taskDataFile = new File(workDirectory + File.separator + taskDataFilename);
            fileInputStream = new FileInputStream(taskDataFile);
            
            ISO11783TaskData taskData = JaxbMarshalUnmarshalUtil.unmarshalwebservice(prefix + parameterMapISOXML.get("isoxmlTaskXsdDatei"), fileInputStream, ISO11783TaskData.class);
            taskManagerLogger.info("createGeoJSONTask-> function: unmarshal" + taskDataFile.getCanonicalPath());
            Feature feature = null;
            String geoJSON = null;
            
            if (taskData != null) {
                List<Object> xmlElementList = taskData.getXmlElements();
                
                HashMap<String, String> pdtNameHashMap = null;
                HashMap<String, String> vpnUnitHashMap = null;
                
                
                //TODO: not nice but it works
                //create relations
                for(Object xmlElement : xmlElementList) {
                    if(xmlElement instanceof VPN) {
                        if(vpnUnitHashMap == null) {
                            vpnUnitHashMap = new HashMap<String, String>();
                        }
                        VPN tmpVPN = (VPN)xmlElement;
                        vpnUnitHashMap.put(tmpVPN.getA(), tmpVPN.getE());
                    } else if(xmlElement instanceof PDT) {
                        if(pdtNameHashMap == null) {
                            pdtNameHashMap = new HashMap<String, String>();
                        }
                        PDT tmpPDT = (PDT)xmlElement;
                        pdtNameHashMap.put(tmpPDT.getA(), tmpPDT.getB());
                    }
                }
                
                TaskInformation tI = new TaskInformation();
                 
                tI.setVersionMajor(taskData.getVersionMajor());
                tI.setVersionMinor(taskData.getVersionMinor());
                tI.setManagementSoftwareManufacturer(taskData.getManagementSoftwareManufacturer());
                tI.setManagementSoftwareVersion(taskData.getManagementSoftwareVersion());
                tI.setDataTransferOrigin(taskData.getDataTransferOrigin());
                
                for(Object xmlElement : xmlElementList) {
                    if(xmlElement instanceof TSK) {
                        TSK tmpTSK = (TSK)xmlElement;
                        tI.setTaskID(Long.valueOf(tmpTSK.getA().replace("TSK", "")));
                        tI.setTaskName(tmpTSK.getB());
                        
                        for(Object tZNOrTIMOrOTP : tmpTSK.getTZNOrTIMOrOTP()) {
                            if(tZNOrTIMOrOTP instanceof TZN) {
                                TZN tmpTZN = (TZN)tZNOrTIMOrOTP;
                                for(Object pLNOrPDV : tmpTZN.getPLNOrPDV()) {
                                    if(pLNOrPDV instanceof PDV) {
                                        PDV tmpPDV = (PDV)pLNOrPDV;
                                        TaskInformationPlantProtection tiPP = new TaskInformationPlantProtection();
                                        tiPP.setName(pdtNameHashMap.get(((PDT)tmpPDV.getC()).getA()));
                                        tiPP.setAmount(tmpPDV.getB());
                                        tiPP.setUnit(vpnUnitHashMap.get(((VPN)tmpPDV.getE()).getA()));
                                        tI.addResourceToPlantProtection(tiPP);
                                    }
                                }
                            }
                        }
                    } else if(xmlElement instanceof CTR) {
                        CTR tmpCTR = (CTR)xmlElement;
                        tI.setCustomerName(tmpCTR.getB());
                    } else if(xmlElement instanceof FRM) {
                        FRM tmpFRM = (FRM)xmlElement;
                        tI.setFarmName(tmpFRM.getB());
                    } else if(xmlElement instanceof CTP) {
                        CTP tmpCTP = (CTP)xmlElement;
                        tI.setFruitGroupName(tmpCTP.getB());
                        for(CVT cvtElement : tmpCTP.getCVT()) {
                            CVT tmpCVT = cvtElement;
                            //TODO: only last fruit would be saved
                            tI.setFruitName(tmpCVT.getB());
                        }
                    } else if(xmlElement instanceof DVC) {
                        for(Object dETOrDPTOrDPDElement : ((DVC)xmlElement).getDETOrDPTOrDPD()) {
                            if(dETOrDPTOrDPDElement instanceof DPT) {
                                DPT tmpDPT = (DPT)dETOrDPTOrDPDElement;
                                //TODO: B = byte[]
                                int tmpDDI = convertByteArrayToInt(tmpDPT.getB());
                                if(tmpDDI == 80) {
                                    tI.setApplicationTotalVolume(tmpDPT.getC());
                                } else if(tmpDDI == 90) {
                                    tI.setYieldTotalMass(tmpDPT.getC());
                                } else if(tmpDDI == 116) {
                                    tI.setTotalArea(tmpDPT.getC());
                                } else if(tmpDDI == 117) {
                                    tI.setEffectiveTotalDistance(tmpDPT.getC());
                                } else if(tmpDDI == 119) {
                                    tI.setEffectiveTotalTime(tmpDPT.getC());
                                //TODO: use certified DDIs - AEF
                                } else if(tmpDDI == 1001) {
                                    tI.setAverageTartaricAcid(tmpDPT.getC());
                                } else if(tmpDDI == 1002) {
                                    tI.setAverageMalicAcid(tmpDPT.getC());
                                } else if(tmpDDI == 1003) {
                                    tI.setAverageGlucose(tmpDPT.getC());
                                } else if(tmpDDI == 1004) {
                                    tI.setAverageFructose(tmpDPT.getC());
                                } 
                            }
                        }
                    } else if(xmlElement instanceof PFD) {
                        PFD tmpPFD = (PFD)xmlElement;
                        tI.setPlainName(tmpPFD.getC());
                        tI.setPlainSize(tmpPFD.getD());
                        //TODO: has to be changed if more than one area
                        for(Object pLNOrLSGOrPNT : tmpPFD.getPLNOrLSGOrPNT()) {
                            if(pLNOrLSGOrPNT instanceof PLN) {
                                PLN tmpPLN = (PLN)pLNOrLSGOrPNT;
                                for(LSG tmpLSG : tmpPLN.getLSG()) {
                                    for(PNT tmpPNT : tmpLSG.getPNT()) {
                                        TaskInformationPoint tIP = new TaskInformationPoint();
                                        tIP.setLatitude(Double.valueOf(tmpPNT.getC()));
                                        tIP.setLongitude(Double.valueOf(tmpPNT.getD()));
                                        tI.addPointToCoordinates(tIP);
                                    }
                                }
                            }
                        }
                    } 
                }
                feature = createGeoJSON(tI, work);
            }
            geoJSON = new ObjectMapper().writerWithDefaultPrettyPrinter().writeValueAsString(feature);
            String absoluteGeoJSONFilename = ieu.getRandomDirectoryExport() + File.separator + geoJsonFilename;
            FileUtils.writeStringToFile(new File(absoluteGeoJSONFilename), geoJSON, charset);
            absoluteGeoJSONFilenameZip = ieu.createZipFilesForExport();
            // Send to DatawareHouse
            if(activateDataStorage && datawareHouseUrl != null && datawareHouseUrl.length() > 0) {
                sendHttpServletResponseOrURL(absoluteGeoJSONFilename, null, datawareHouseUrl, "application/json");
            }
        } catch (JAXBException jbe) {
            taskManagerLogger.error("createGeoJSONTask(JAXBException)->" + jbe.getMessage(),jbe);
        } catch (SAXException saxe) {
            taskManagerLogger.error("createGeoJSONTask(SAXException)->" + saxe.getMessage(),saxe);
        } catch (XMLException e) {
            taskManagerLogger.error("createGeoJSONTask(XMLxception)->" + e.getMessage(),e);
        } catch (IOException e) {
            taskManagerLogger.error("createGeoJSONTask(IOException)->" + e.getMessage(),e);
        } catch (Exception e) {
            taskManagerLogger.error("createGeoJSONTask(Exception)->" + e.getMessage(), e);
        } finally {
            try {
                fileInputStream.close();
                ieu.deleteImportFiles();
            } catch (IOException e) {
                taskManagerLogger.error("createGeoJSONTask->fileInputStream can't be closed.->" + e.getMessage());
            }
        }
        return absoluteGeoJSONFilenameZip;
    }
    
    private int convertByteArrayToInt(byte[] bytes) {
        return  ((bytes[0] & 0xFF) << 16) |
                ((bytes[1] & 0xFF) << 0);
    }
    
    private void createPlainTextToDownload(HttpServletRequest request, HttpServletResponse response, String fileName, String fileContent) throws FileException {
        taskManagerLogger.info("createPlainTextToDownload->function");
        ServletOutputStream servletOutputStream = null;
        ByteArrayInputStream bai = null;
        try {
            servletOutputStream = response.getOutputStream();
            response.setContentType("text/plain");
            response.setContentLength(fileContent.length());
            response.setHeader("Content-Disposition", "attachment;filename="+fileName);

            bai = new ByteArrayInputStream(fileContent.getBytes());
            byte[] bbuf = new byte[1024];
            int len = 0;
            while ((len = bai.read(bbuf)) != -1) {
                servletOutputStream.write(bbuf, 0, len);
            }
        } catch (IOException ioe) {
            throw new FileException("createPlainTextToDownload->" + ioe.getMessage());
        } finally {
            try {
                bai.close();
                servletOutputStream.flush();
                servletOutputStream.close();
            } catch (IOException e) {
                taskManagerLogger.error("createPlainTextToDownload->"+e.getMessage());
            }
        }
    }
    
    private void sendHttpServletResponseOrURL(String absoluteFileName, HttpServletResponse response, String url, String contentType) {
        File file = new File(absoluteFileName);
        HttpURLConnection httpcon = null;
        FileInputStream is = null;
        OutputStream os = null;
       
        try {
            if(response != null) {
                taskManagerLogger.info("send to response");
                //response.setContentType("multipart/form-data");
                response.setContentType(contentType);
                response.setHeader("Content-Disposition", "attachment; filename=" + file.getName());
                os = response.getOutputStream();
            } else if(url != null){
                taskManagerLogger.info("send to url ->" + url);
                httpcon = (HttpURLConnection) new URL(url).openConnection();
                httpcon.setRequestMethod("POST");
                httpcon.setDoOutput(true);
                httpcon.setUseCaches(false);
                //httpcon.setRequestProperty("Content-Type", "multipart/form-data");
                httpcon.setRequestProperty("Content-Type", contentType);
                httpcon.setRequestProperty("Content-Disposition", "attachment; filename=" + file.getName());
                httpcon.connect();
                os = httpcon.getOutputStream();
            }
            is = new FileInputStream(file);
            byte[] buffer = new byte[4096];
            int bytes_read; // How many bytes in buffer
            while ((bytes_read = is.read(buffer)) != -1) {
                os.write(buffer, 0, bytes_read);
            }
            //Response
            if(url != null) {
                int responseCode = httpcon.getResponseCode();
                String responseMessage = httpcon.getResponseMessage();
                taskManagerLogger.info("responseCode:" + responseCode + ";responseMessage:"+responseMessage);
                BufferedReader br = null;
                if (100 <= httpcon.getResponseCode() && httpcon.getResponseCode() <= 399) {
                    br = new BufferedReader(new InputStreamReader(httpcon.getInputStream()));
                } else {
                    br = new BufferedReader(new InputStreamReader(httpcon.getErrorStream()));
                }
                if(br != null) {
                    String strCurrentLine;
                    while ((strCurrentLine = br.readLine()) != null) {
                        taskManagerLogger.info("responseBody:" + strCurrentLine);
                    }
                    
                }
            }
        } catch (MalformedURLException e) {
            taskManagerLogger.error(e.getMessage());
        } catch (IOException e) {
            taskManagerLogger.error(e.getMessage());
        } finally {
            try {
                is.close();
            } catch (Exception e) {
            }
            try {
                os.close();
            } catch (Exception e) {
            }
        }
    }
    
    private ISO11783TaskData createTaskData(TaskInformation tI, String work){
        taskManagerLogger.info("function[createTaskData]");
        ISO11783TaskData taskData = new ISO11783TaskData();
        List<Object> xmlElements = taskData.getXmlElements();
       
        taskData.setVersionMajor(tI.getVersionMajor());
        taskData.setVersionMinor(tI.getVersionMinor());
        taskData.setManagementSoftwareManufacturer(tI.getManagementSoftwareManufacturer());
        taskData.setManagementSoftwareVersion(tI.getManagementSoftwareVersion());
        taskData.setDataTransferOrigin(tI.getDataTransferOrigin());
        
        CTR ctr = new CTR();
        ctr.setA("CTR" + createSubstringForID("1", 11));      
        ctr.setB(createSubstring(tI.getCustomerName(), 32));
        xmlElements.add(ctr);
      
        FRM frm = new FRM();
        frm.setA("FRM" + createSubstringForID("1", 11));
        frm.setB(createSubstring(tI.getFarmName(), 32));
        xmlElements.add(frm);
       
        CTP ctp = new CTP();
        ctp.setA("CTP" + createSubstringForID("1", 11));
        ctp.setB(createSubstring(tI.getFruitGroupName(), 32));
        CVT cvt = new CVT();
        cvt.setA("CVT" + createSubstringForID("1", 11));
        cvt.setB(createSubstring(tI.getFruitName(), 32));
        ctp.addCVT(cvt);
        xmlElements.add(ctp);

        PFD pfd = new PFD();
        pfd.setA("PFD" + createSubstringForID("1", 11));
        pfd.setC(createSubstring(tI.getPlainName(), 32));
        pfd.setD(tI.getPlainSize());
        pfd.setE(ctr);
        pfd.setF(frm);
        pfd.setG(ctp);
        pfd.setH(cvt); 
        xmlElements.add(pfd);
        
        if (tI.getCoordinates() != null && !tI.getCoordinates().isEmpty()) {
            PLN pln = new PLN();
            pln.setA("1");
            pfd.getPLNOrLSGOrPNT().add(pln);
            
            LSG lsg = new LSG();
            lsg.setA("1");
            pln.getLSG().add(lsg);
            
            PNT pnt = null;
            for (TaskInformationPoint tIP : tI.getCoordinates()) {
                pnt = new PNT();
                pnt.setA("2");
                //fractionDigits 9
                pnt.setC(String.valueOf(round(tIP.getLatitude(),9)));
                pnt.setD(String.valueOf(round(tIP.getLongitude(),9)));
                lsg.getPNT().add(pnt);
            }
        }
        
        TSK tsk = new TSK();
        tsk.setA("TSK" + createSubstringForID(tI.getTaskID(), 11));
        tsk.setB(createSubstring(tI.getTaskName(), 32));
        tsk.setC(ctr);
        tsk.setE(pfd);
        tsk.setG("1");
        xmlElements.add(tsk);
        
        if(work.equals(workSpray)) {
            //TODO: to much specified hard coded
            if (tI.getPlantProtection() != null && !tI.getPlantProtection().isEmpty()) {
                PGP pgp = new PGP();
                pgp.setA("PGP" + createSubstringForID("1", 11));
                pgp.setB(createSubstring("Plant Protection", 32));
                xmlElements.add(pgp);
                
                VPN vpnLiter = new VPN();
                vpnLiter.setA("VPN" + createSubstringForID("1", 11));
                vpnLiter.setB(0);
                vpnLiter.setC(BigDecimal.valueOf(1.0));
                vpnLiter.setD((short)3);
                vpnLiter.setE(plantProtectionUnitLiter);
                xmlElements.add(vpnLiter);
                
                VPN vpnKG = new VPN();
                vpnKG.setA("VPN" + createSubstringForID("2", 11));
                vpnKG.setB(0);
                vpnKG.setC(BigDecimal.valueOf(1.0));
                vpnKG.setD((short)3);
                vpnKG.setE(plantProtectionUnitGramm);
                xmlElements.add(vpnKG);
                
                TZN tzn = new TZN();
                tzn.setA((short)0);
                tsk.getTZNOrTIMOrOTP().add(tzn);
                
                //TODO: Die EInbindung der PDTs funktioniert noch nicht
                PDT pdtWater = new PDT();
                pdtWater.setA("PDT" + createSubstringForID("100", 11));
                pdtWater.setB("Water");
                pdtWater.setD(vpnLiter);
                xmlElements.add(pdtWater);
                
                int uniqueID = 1;
                PDT pdt = null;
                PDV pdv = null;
                for (TaskInformationPlantProtection tIPP : tI.getPlantProtection()) {
                    pdt = new PDT();
                    pdt.setA("PDT" + createSubstringForID(Integer.valueOf(uniqueID), 11));
                    pdt.setB(tIPP.getName());
                    pdt.setC(pgp);
                    if(tIPP.getUnit() != null) {
                        switch(tIPP.getUnit()){
                            case plantProtectionUnitLiter:
                                pdt.setD(vpnLiter);
                                break;
                            case plantProtectionUnitGramm:
                                pdt.setD(vpnKG);
                                break;
                            default:
                                pdt.setD(vpnLiter);
                                break; 
                        }
                    }
                    xmlElements.add(pdt);
                    
                    pdv = new PDV();
                    pdv.setA(toByteArray("0001"));
                    pdv.setB(tIPP.getAmount());
                    pdv.setC(pdt);
                    pdv.setE(pdt.getD());
                    tzn.getPLNOrPDV().add(pdv);

                    uniqueID++;
                }
               
            }
        }
        //TODO: implement tlg
        // Log
        //TLG tlg = new TLG();
        //tlg.setA(getTlgFilename());
        //tlg.setC("1");
        // tsk.getCNNOrDANOrDLT().add(tlg);

        return taskData;
    }
    
    private byte[] toByteArray(String s) {
        return DatatypeConverter.parseHexBinary(s);
    }
   
    private Feature createGeoJSON(TaskInformation tI, String work){
        taskManagerLogger.info("function[createGeoJSONJackson]");
        Feature feature = new Feature();
       
        Map<String, Object> properties = new HashMap<String, Object>();
        properties.put("task_id", tI.getTaskID());
        properties.put("task_name", tI.getTaskName());
        properties.put("customer_name", tI.getCustomerName());
        properties.put("farm_name", tI.getFarmName());
        properties.put("fruit_group_name", tI.getFruitGroupName());
        properties.put("fruit_name", tI.getFruitName());
        properties.put("plain_name", tI.getPlainName());
        properties.put("plain_size", tI.getPlainSize());
        properties.put("total_area", tI.getTotalArea());
        properties.put("effective_total_distance", tI.getEffectiveTotalDistance());
        properties.put("effective_total_time", tI.getEffectiveTotalTime());
        if(work.equals(workHarvest)) {
            properties.put("yield_total_mass", tI.getYieldTotalMass());
            properties.put("average_tartaric_acid", tI.getAverageTartaricAcid());
            properties.put("average_malic_acid", tI.getAverageMalicAcid());
            properties.put("average_glucose", tI.getAverageGlucose());
            properties.put("average_fructose", tI.getAverageFructose());
        } else if(work.equals(workSpray)) {
            ObjectMapper mapper = new ObjectMapper();
            ArrayNode arrayNode = null;
            ArrayList<TaskInformationPlantProtection> tIPPArrayList = tI.getPlantProtection();
            if(tIPPArrayList != null && !tIPPArrayList.isEmpty()){
                arrayNode = mapper.createArrayNode();
                for(TaskInformationPlantProtection tIPP : tIPPArrayList) {
                    ObjectNode tmpObjectNode = mapper.createObjectNode();
                    tmpObjectNode.put("name", tIPP.getName());
                    tmpObjectNode.put("amount", tIPP.getAmount());
                    tmpObjectNode.put("unit", tIPP.getUnit());
                    arrayNode.add(tmpObjectNode);
                }
            }
            properties.put("plant_protection", arrayNode != null ? arrayNode : null);
            properties.put("application_total_volume", tI.getApplicationTotalVolume());
        }
        feature.setProperties(properties);
        
        Polygon polygon = new Polygon();
        if(tI.getCoordinates() != null && !tI.getCoordinates().isEmpty()) {
            List<LngLatAlt> coordinatesList = new ArrayList<LngLatAlt>();
            for(TaskInformationPoint tIP : tI.getCoordinates()) {
                LngLatAlt tmpLngLatAlt = new LngLatAlt();
                tmpLngLatAlt.setLongitude(tIP.getLongitude());
                tmpLngLatAlt.setLatitude(tIP.getLatitude());
                coordinatesList.add(tmpLngLatAlt);
            }
            List<List<LngLatAlt>> list = new ArrayList<List<LngLatAlt>>();
            list.add(coordinatesList);
            polygon.setCoordinates(list);
        } 
        feature.setGeometry(polygon);
        
        return feature;
    }
    
    private String createSubstring(Object object, int length) {
        if (object != null) {
            String string = object.toString();
            if (string.length() < length) {
                length = string.length();
            }
            return string.substring(0, length);
        } else {
            return null;
        }

    }
    
    private String createSubstringForID(Object object, int length) {
        if (object != null && object.toString().length() > 0) {
            String string = object.toString();
            if (string.length() < length) {
                length = string.length();
            }
            return string.substring(string.length() - length, string.length());
        } else {
            return "";
        }

    }

    @SuppressWarnings("unused")
    private String getCurrentDatumString(boolean withMilliseconds) {
        Calendar calendar = new GregorianCalendar();
        calendar.setTimeZone(TimeZone.getTimeZone("CET"));

        String date = String.valueOf(calendar.get(Calendar.YEAR));
        date += addLeadingZero(String.valueOf(calendar.get(Calendar.MONTH) + 1));
        date += addLeadingZero(String.valueOf(calendar.get(Calendar.DATE))) + "_";
        date += addLeadingZero(String.valueOf(calendar.get(Calendar.HOUR_OF_DAY)));
        date += addLeadingZero(String.valueOf(calendar.get(Calendar.MINUTE)));
        date += addLeadingZero(String.valueOf(calendar.get(Calendar.SECOND)));
        if (withMilliseconds == true) {
            date += "_" + String.valueOf(calendar.get(Calendar.MILLISECOND));
        }
        return date;
    }
    
    private String addLeadingZero(String tempStrng) {
        if (tempStrng != null && tempStrng.length() == 1) {
            return "0".concat(tempStrng);
        } else {
            return tempStrng;
        }
    }
    
    
    
    @SuppressWarnings("unused")
    private String getTlgFilename() {
        if (tlgFilename.intValue() < 100000) {
            tlgFilename = Integer.valueOf(tlgFilename.intValue() + 1);
        } else {
            tlgFilename = Integer.valueOf(0);
        }
        int possibleNumbers = 5;
        int zeroNumersToAdd = possibleNumbers - tlgFilename.toString().length();

        String filename = "TLG";
        for (; zeroNumersToAdd > 0; zeroNumersToAdd--) {
            filename = filename.concat("0");
        }
        return filename.concat(tlgFilename.toString());
    }
    
    private double round(final double value, final int frac) {
        return Math.round(Math.pow(10.0, frac) * value) / Math.pow(10.0, frac);
    }
    
    @Override
    public String getPackageClassName() {
        return TaskManager.class.getName();
    }
}
